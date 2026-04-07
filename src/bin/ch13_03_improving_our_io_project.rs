use helloworld::{print_line_separator, search, search_case_insensitive};
use std::error::Error;
use std::{env, fs, process};

//cargo run --bin ch13_03_improving_our_io_project -- You example-filename.txt

// 演示目的：使用迭代器改进 I/O 项目的性能
//
// demo_1: 传统方式 - 使用 Vec 和 slice
// - 使用 env::args().collect() 将参数收集到 Vec<String>
// - Config::build 接受 &[String] 切片作为参数
// - 需要使用 clone() 获取字符串所有权
// - 缺点：额外的内存分配和字符串克隆开销
//
// demo_2: 迭代器方式 - 零成本抽象
// - 直接使用 env::args() 迭代器，无需 collect()
// - Config::build 接受 impl Iterator<Item = String> 作为参数
// - 使用 next() 直接获取字符串所有权，无需 clone
// - 优点：更高效的内存使用，避免不必要的分配和克隆
//
// 核心改进：
// 1. 消除中间 Vec 分配
// 2. 消除字符串 clone 操作
// 3. 利用迭代器的惰性求值特性

fn demo_1() {
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }

    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let query = args[1].clone();
            let file_path = args[2].clone();

            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }
        Ok(())
    }

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn demo_2() {
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }

    impl Config {
        fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
            args.next();
            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let file_path = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path"),
            };

            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };
        for line in results {
            println!("{line}");
        }
        Ok(())
    }

    // 改进1：直接使用迭代器，无需先收集到 Vec
    // demo_1: let args: Vec<String> = env::args().collect(); - 会分配额外内存
    // demo_2: let args = env::args(); - 零成本抽象，惰性求值
    let args = env::args();

    // 改进2：直接传递迭代器给 build 方法，避免了 clone 操作
    // demo_1: Config::build(&args) - 参数类型是 &[String]，build 内部需要 clone
    // demo_2: Config::build(args) - 参数类型是 impl Iterator<Item = String>，通过 next() 获取所有权，无需 clone
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn main() {
    demo_1();
    println!();
    print_line_separator();
    demo_2();
}
