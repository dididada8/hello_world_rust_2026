use helloworld::print_type_of;

fn demo_1() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
    print_type_of(&c, Some("demo_1:CustomSmartPointer"));
    print_type_of(&d, Some("demo_1:CustomSmartPointer"));
    println!("CustomSmartPointers will go out of scope soon...\n");
}

fn main() {
    demo_1();
}
