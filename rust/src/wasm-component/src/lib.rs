wit_bindgen::generate!({
    world: "hello-world",
    path:"../../wit"
});

mod another_component {
    wit_bindgen::generate!({
        world: "byebye-world",
        path:"../../wit"
    });

    // struct ByeBye;

    // impl ByebyeWorld for ByeBye {
    //     fn byebye() -> wit_bindgen::rt::string::String {
    //         "Bye bye".to_string()
    //     }
    // }

    // export_byebye_world!(ByeBye);
}

// Define a custom type and implement the generated `Host` trait for it which
// represents implementing all the necesssary exported interfaces for this
// component.
struct MyHost;

impl HelloWorld for MyHost {
    fn greet() -> String {
        println!(
            "Hi {}, I'm greeting you from the guest in a println!",
            name()
        );

        format!("Hi {}, I'm greeting you from the guest!", name())
    }
}

export_hello_world!(MyHost);
