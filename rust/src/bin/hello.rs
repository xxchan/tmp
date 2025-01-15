
#[tokio::main]
async fn main() {
    // my_function(MyStruct).await;


    let mut foo = Foo { x: "hello".to_string() };

    let x = &mut foo.x;
    foo.hi();
    *x = "world".to_string();

}

// async fn my_function(t: impl MyTrait) {
//     println!("{}", t.my_method().await);
// }

#[async_trait::async_trait]
trait MyTrait:Send {
    async fn my_method(&self) -> &str {
        "default"
    }
}

struct MyStruct;

#[async_trait::async_trait]
impl MyTrait for MyStruct {
    async fn my_method(&self) -> &str {
        "impl"
    }
}


struct Foo {
    x: String
}

impl Foo {
    fn hi(&self) {
        println!("{}", self.x);
    }
}