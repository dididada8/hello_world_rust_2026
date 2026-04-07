use helloworld::{print_line_separator, print_type_of};

fn demo_1(){
    #[derive(Debug)]
    #[allow(dead_code)]
    enum List{
        Cons(i32, Box<List>),
        Nil
    }
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print_type_of(&list,Some("box list demo"));


}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    print_type_of(&b,Some("box demo"));

    print_line_separator();
    demo_1();
}