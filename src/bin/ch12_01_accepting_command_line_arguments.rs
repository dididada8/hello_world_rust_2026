use std::env;

//cargo run --bin ch12_01_accepting_command_line_arguments -- searchstring example-filename.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

}