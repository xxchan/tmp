use expect_test::expect;
use serde::de::{Deserializer, Error as _, IntoDeserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
struct Properties {
    f1: Option<String>,
    f2: Option<u8>,
    #[serde(flatten, deserialize_with = "deserialize_fields_nested")]
    nested: HashMap<String, Properties>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Foo {
    a: usize,
    b: String,
    #[serde(flatten, deserialize_with = "deserialize_fields")]
    fields: HashMap<String, Properties>,
}

struct FieldsVisitor {
    nested: bool,
}

impl<'de> Visitor<'de> for FieldsVisitor {
    type Value = HashMap<String, Properties>;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("map with dotted keys")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut temp = HashMap::<String, HashMap<String, serde_value::Value>>::new();

        while let Some((key, value)) = map.next_entry::<String, serde_value::Value>()? {
            let (k1, k2);
            if !self.nested {
                let fields;
                [fields, k1, k2] =
                    key.splitn(3, '.')
                        .collect::<Vec<_>>()
                        .try_into()
                        .map_err(|_| {
                            M::Error::custom(format_args!(
                                "expected key in format `fields.<name>.<property>`, got `{key}`"
                            ))
                        })?;
                if fields != "fields" {
                    return Err(M::Error::custom(format_args!(
                        "expected key in format `fields.<name>.<property>`, got `{key}`"
                    )));
                }
            } else {
                [k1, k2] = key
                    .splitn(2, '.')
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| {
                        M::Error::custom(
                            format_args!("expected key in format `<name>.<property>`, got `{key}`")
                        )
                    })?;
            };

            temp.entry(k1.to_owned())
                .or_default()
                .insert(k2.to_owned(), value);
        }

        temp.into_iter()
            .map(|(k1, vals)| {
                let vals =
                    Properties::deserialize(vals.into_deserializer()).map_err(|e| {
                        M::Error::custom(format_args!("failed to deserialize `fields.{k1}`: {e}"))
                    })?;
                Ok((k1, vals))
            })
            .collect()
    }
}

fn deserialize_fields<'de, D>(deserializer: D) -> Result<HashMap<String, Properties>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(FieldsVisitor { nested: false })
}

fn deserialize_fields_nested<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Properties>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(FieldsVisitor { nested: true })
}

// test deny_unknown_fields and flatten

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Foo1 {
    #[serde(flatten)]
    flatten: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Foo2 {
    #[serde(flatten)]
    flatten: HashMap<String, String>,

    #[serde(flatten)]
    flatten2: HashMap<String, String>,

    #[serde(flatten)]
    flatten3: FooInner,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct FooInner {
    a: Option<String>,
    b: Option<String>,
}

#[test]
fn test_1() {
    let json = r#"{
       "a": "b", "c":"d"
    }"#;

    let foo: Result<Foo1, _> = serde_json::from_str(json);
    let foo2: Result<Foo2, _> = serde_json::from_str(json);

    // https://serde.rs/container-attrs.html#deny_unknown_fields
    // > Note: this attribute is not supported in combination with flatten, neither on the outer struct nor on the flattened field.

    // With outer `deny_unknown_fields`, flatten is ignored
    expect![[r#"
        Err(
            Error("unknown field `a`", line: 3, column: 5),
        )
    "#]]
    .assert_debug_eq(&foo);

    // When there are multiple flatten, all of them will be used.
    // Also, with outer `flatten``, the inner `deny_unknown_fields` is ignored.
    expect![[r#"
        Ok(
            Foo2 {
                flatten: {
                    "c": "d",
                    "a": "b",
                },
                flatten2: {
                    "a": "b",
                    "c": "d",
                },
                flatten3: FooInner {
                    a: Some(
                        "b",
                    ),
                    b: None,
                },
            },
        )
    "#]]
    .assert_debug_eq(&foo2);
}

fn main() {
    let json = r#"{
        "a": 1,
        "b": "hello",
        "fields.a.f1": "v1",
        "fields.a.f2": 0,
        "fields.b.f1": "v3",
        "fields.a.f3.f4": "v4"
    }"#;

    let foo: Foo = serde_json::from_str(json).unwrap();

    println!("{:#?}", foo);
}
