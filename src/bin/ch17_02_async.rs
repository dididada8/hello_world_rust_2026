fn demo_1() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec!["hi", "from", "the", "future"];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received: {}", value);
        }
    });
}

fn main() {
    demo_1();
}
