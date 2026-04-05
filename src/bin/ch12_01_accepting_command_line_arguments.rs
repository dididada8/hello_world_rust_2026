use helloworld::print_line_separator;
use std::{env, fs};
//cargo run --bin ch12_01_accepting_command_line_arguments -- searchstring example-filename.txt

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

fn demo_1() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn demo_2() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);
    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!(
        "With text:\n{}",
        fs::read_to_string(config.file_path).expect("Should have been able to read the file")
    );
    let args: &[String] = &args[..3];
    let config = Config::new(args);
    println!("{:?},{:?}", args, config);
}

fn main() {
    demo_1();
    print_line_separator();
    println!();
    demo_2();
}

fn check_args(args: &Vec<String>) {
    if args.len() < 3 {
        eprintln!(
            "Usage: cargo run --bin ch12_01_accepting_command_line_arguments -- <query> <file_path> \n\
             Example: cargo run --bin ch12_01_accepting_command_line_arguments -- searchstring example-filename.txt"
        );
        std::process::exit(1);
    }
}
