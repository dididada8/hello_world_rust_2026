use helloworld::{print_line_separator, print_type_of};
use std::thread;

fn demo_1() {
    let channel = thread::spawn(|| {
        for i in 1..=5 {
            println!("__hi number {i} from the channel thread!");
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..=5 {
                println!("hi number {i} from the spawned task!");
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        for i in 1..=5 {
            println!("hi number {i} from the main task!");
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }
        print_type_of(&handle, Some("demo_1:JoinHandle<()>"));
    });


    channel.join().unwrap();
}

fn main() {
    demo_1();
    print_line_separator();
}
