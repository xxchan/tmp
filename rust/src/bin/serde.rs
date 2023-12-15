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
                        M::Error::custom(format_args!(
                            "expected key in format `<name>.<property>`, got `{key}`"
                        ))
                    })?;
            };

            temp.entry(k1.to_owned())
                .or_default()
                .insert(k2.to_owned(), value);
        }

        temp.into_iter()
            .map(|(k1, vals)| {
                let vals = Properties::deserialize(vals.into_deserializer()).map_err(|e| {
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

mod unkown_fields_and_flatten {
    use super::*;

    // test deny_unknown_fields and flatten

    // https://serde.rs/container-attrs.html#deny_unknown_fields
    // > Note: this attribute is not supported in combination with flatten, neither on the outer struct nor on the flattened field.

    // https://github.com/serde-rs/serde/issues/1547#issuecomment-705744778
    // The cause for this is that the way #[serde(flatten)] works is that it deserializes the container as a map instead of a struct, and holds a map of unknown entries as it goes through. Then at the end it uses the field's Deserialize implementation to deserialize from said map.
    // (nested flatten doesn't work. Guess this is because nested flatten will make a struct deserialize as a map.)

    // TL;DR: deny_unknown_fields
    // - doesn't work with flatten map
    // - can work with flatten struct
    // - doesn't work with nested flatten struct (This makes a flatten struct behave like a flatten map)

    #[test]
    fn test_outer_deny() {
        #[derive(Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct FlattenMap {
            #[serde(flatten)]
            flatten: HashMap<String, String>,
        }
        #[derive(Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct FlattenStruct {
            #[serde(flatten)]
            flatten_struct: Inner,
        }

        #[derive(Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct FlattenBoth {
            #[serde(flatten)]
            flatten: HashMap<String, String>,
            #[serde(flatten)]
            flatten_struct: Inner,
        }

        #[derive(Deserialize, Debug)]
        struct Inner {
            a: Option<String>,
            b: Option<String>,
        }

        let json = r#"{
                "a": "b"
            }"#;
        let foo: Result<FlattenMap, _> = serde_json::from_str(json);
        let foo1: Result<FlattenStruct, _> = serde_json::from_str(json);
        let foo2: Result<FlattenBoth, _> = serde_json::from_str(json);

        // with `deny_unknown_fields`, we can't flatten ONLY a map
        expect![[r#"
                Err(
                    Error("unknown field `a`", line: 3, column: 13),
                )
            "#]]
        .assert_debug_eq(&foo);

        // but can flatten a struct!
        expect![[r#"
                Ok(
                    FlattenStruct {
                        flatten_struct: Inner {
                            a: Some(
                                "b",
                            ),
                            b: None,
                        },
                    },
                )
            "#]]
        .assert_debug_eq(&foo1);
        // unknown fields can be denied.
        let foo11: Result<FlattenStruct, _> = serde_json::from_str(r#"{ "a": "b", "unknown":1 }"#);
        expect_test::expect![[r#"
                Err(
                    Error("unknown field `unknown`", line: 1, column: 25),
                )
            "#]]
        .assert_debug_eq(&foo11);

        // When both struct and map are flattened, the map also works...
        expect![[r#"
                Ok(
                    FlattenBoth {
                        flatten: {
                            "a": "b",
                        },
                        flatten_struct: Inner {
                            a: Some(
                                "b",
                            ),
                            b: None,
                        },
                    },
                )
            "#]]
        .assert_debug_eq(&foo2);

        let foo21: Result<FlattenBoth, _> = serde_json::from_str(r#"{ "a": "b", "unknown":1 }"#);
        expect_test::expect![[r#"
                Err(
                    Error("invalid type: integer `1`, expected a string", line: 1, column: 25),
                )
            "#]]
        .assert_debug_eq(&foo21);
        // This error above is a little funny, since even if we use string, it will still fail.
        let foo22: Result<FlattenBoth, _> = serde_json::from_str(r#"{ "a": "b", "unknown":"1" }"#);
        expect_test::expect![[r#"
                Err(
                    Error("unknown field `unknown`", line: 1, column: 27),
                )
            "#]]
        .assert_debug_eq(&foo22);
    }

    #[test]
    fn test_inner_deny() {
        // no outer deny now.
        #[derive(Deserialize, Debug)]
        struct FlattenStruct {
            #[serde(flatten)]
            flatten_struct: Inner,
        }
        #[derive(Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct Inner {
            a: Option<String>,
            b: Option<String>,
        }

        let json = r#"{
                "a": "b", "unknown":1
            }"#;
        let foo: Result<FlattenStruct, _> = serde_json::from_str(json);
        // unknown fields cannot be denied.
        // I think this is because `deserialize_struct` is called, and required fields are passed.
        // Other fields are left for the outer struct to consume.
        expect_test::expect![[r#"
                Ok(
                    FlattenStruct {
                        flatten_struct: Inner {
                            a: Some(
                                "b",
                            ),
                            b: None,
                        },
                    },
                )
            "#]]
        .assert_debug_eq(&foo);
    }

    #[test]
    fn test_multiple_flatten() {
        #[derive(Deserialize, Debug)]
        struct Foo {
            /// struct will "consume" the used fields!
            #[serde(flatten)]
            flatten_struct: Inner1,

            /// map will keep the unknown fields!
            #[serde(flatten)]
            flatten_map1: HashMap<String, String>,

            #[serde(flatten)]
            flatten_map2: HashMap<String, String>,

            #[serde(flatten)]
            flatten_struct2: Inner2,
        }

        #[derive(Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct Inner1 {
            a: Option<String>,
            b: Option<String>,
        }
        #[derive(Deserialize, Debug)]
        struct Inner11 {
            c: Option<String>,
        }
        #[derive(Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct Inner2 {
            c: Option<String>,
        }

        let json = r#"{
                "a": "b", "c":"d"
            }"#;
        let foo2: Result<Foo, _> = serde_json::from_str(json);

        // When there are multiple flatten, all of them will be used.
        // Also, with outer `flatten``, the inner `deny_unknown_fields` is ignored.
        expect![[r#"
            Ok(
                Foo {
                    flatten_struct: Inner1 {
                        a: Some(
                            "b",
                        ),
                        b: None,
                    },
                    flatten_map1: {
                        "c": "d",
                    },
                    flatten_map2: {
                        "c": "d",
                    },
                    flatten_struct2: Inner2 {
                        c: Some(
                            "d",
                        ),
                    },
                },
            )
        "#]]
        .assert_debug_eq(&foo2);
    }

    #[test]
    fn test_nested_flatten() {
        #[derive(Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct Outer {
            #[serde(flatten)]
            inner: Inner,
        }

        #[derive(Deserialize, Debug)]
        struct Inner {
            a: Option<String>,
            b: Option<String>,
            #[serde(flatten)]
            nested: InnerInner,
        }

        #[derive(Deserialize, Debug)]
        struct InnerInner {
            c: Option<String>,
        }

        let json = r#"{ "a": "b", "unknown":"1" }"#;

        let foo: Result<Outer, _> = serde_json::from_str(json);

        // This is very unfortunate...
        expect_test::expect![[r#"
            Err(
                Error("unknown field `a`", line: 1, column: 27),
            )
        "#]]
        .assert_debug_eq(&foo);

        // Actually, the nested `flatten` will makes the struct behave like a map.
        // Let's remove `deny_unknown_fields` and see
        #[derive(Deserialize, Debug)]
        struct Outer2 {
            #[serde(flatten)]
            inner: Inner,
            /// We can see the fields of `inner` are not consumed.
            #[serde(flatten)]
            map: HashMap<String, String>,
        }
        let foo2: Result<Outer2, _> = serde_json::from_str(json);
        expect_test::expect![[r#"
            Ok(
                Outer2 {
                    inner: Inner {
                        a: Some(
                            "b",
                        ),
                        b: None,
                        nested: InnerInner {
                            c: None,
                        },
                    },
                    map: {
                        "unknown": "1",
                        "a": "b",
                    },
                },
            )
        "#]]
        .assert_debug_eq(&foo2);
    }

    #[test]
    fn test_flatten_option() {
        #[derive(Deserialize, Debug)]
        struct Foo {
            /// flatten option struct can still consume the field
            #[serde(flatten)]
            flatten_struct: Option<Inner1>,

            /// flatten option map is always `Some`
            #[serde(flatten)]
            flatten_map1: Option<HashMap<String, String>>,

            /// flatten option struct is `None` if the required field is absent
            #[serde(flatten)]
            flatten_struct2: Option<Inner2>,

            /// flatten option struct is `Some` if the required field is present and optional field is absent
            #[serde(flatten)]
            flatten_struct3: Option<Inner3>,
        }

        #[derive(Deserialize, Debug)]
        struct Inner1 {
            a: Option<String>,
            b: Option<String>,
        }
        #[derive(Deserialize, Debug)]
        struct Inner11 {
            c: Option<String>,
        }

        #[derive(Deserialize, Debug)]
        struct Inner2 {
            c: Option<String>,
            d: String,
        }

        #[derive(Deserialize, Debug)]
        struct Inner3 {
            e: Option<String>,
            f: String,
        }

        #[derive(Deserialize, Debug)]
        struct Inner4 {
            g: Option<String>,
        }

        let json = r#"{
        "a": "b", "c": "d", "f": "g"
     }"#;
        let foo: Result<Foo, _> = serde_json::from_str(json);
        expect![[r#"
            Ok(
                Foo {
                    flatten_struct: Some(
                        Inner1 {
                            a: Some(
                                "b",
                            ),
                            b: None,
                        },
                    ),
                    flatten_map1: Some(
                        {
                            "c": "d",
                            "f": "g",
                        },
                    ),
                    flatten_struct2: None,
                    flatten_struct3: Some(
                        Inner3 {
                            e: None,
                            f: "g",
                        },
                    ),
                },
            )
        "#]]
        .assert_debug_eq(&foo);
    }
}
