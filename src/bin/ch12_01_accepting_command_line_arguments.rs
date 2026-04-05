use std::{env, fs};

//cargo run --bin ch12_01_accepting_command_line_arguments -- searchstring example-filename.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: cargo run --bin ch12_01_accepting_command_line_arguments -- <query> <file_path> \n\
             Example: cargo run --bin ch12_01_accepting_command_line_arguments -- searchstring example-filename.txt"
        );
        std::process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
