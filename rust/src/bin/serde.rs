#[derive(Deserialize)]
pub struct Foo {
    a: usize,
    pub b: String,
    fields: HashMap<String, String>,
}

fn main() {
    let foo: Foo =
        serde_json::from_str(r#"{"a": 1, "b": "hello", "fields": {"c": "world"}}"#).unwrap();
    println!("{:?}", foo);
}
