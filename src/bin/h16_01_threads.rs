use helloworld::print_line_separator;
use std::thread;
use std::time::Duration;

fn demo_1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned_thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("HI number {i} from the main=thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    demo_1();
    print_line_separator();
}
