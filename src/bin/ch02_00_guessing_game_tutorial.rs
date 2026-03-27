use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::RngExt;

fn main() {

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Guess the number!");
    println!("Please input your guess within 10 seconds.");

    // 创建一个 mpsc（多生产者、单消费者）通道：
    // tx = transmitter（发送端），在子线程里把读取到的输入字符串 send 出去
    // rx = receiver（接收端），在主线程里接收输入，并可配合超时等待
    let (tx, rx) = mpsc::channel::<String>();

    // 子线程负责阻塞读取标准输入，避免主线程被 read_line 卡住
    thread::spawn(move || {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let _ = tx.send(guess);
    });

    // 主线程最多等待 10 秒；超时就直接继续执行后续代码
    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(guess) => println!("You guessed: {}", guess.trim_end()),
        Err(_) => println!("10 seconds passed with no input. Continue..."),
    }

    println!("Goodbye!");

    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
}
