use crate::ch12_05_working_with_environment_variables::{
    Config, run,
};
use helloworld::{search, search_case_insensitive};
use std::{env, process};

#[cfg(test)]
mod tests {
    use helloworld::{search, search_case_insensitive};

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

fn main() {
    let query = "duct";
    let contents = "";
    println!("{:?}", search(query, contents));

    let query = "rUsT";
    let contents = "";
    println!("{:?}", search_case_insensitive(query, contents));


    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args[..]).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

mod ch12_05_working_with_environment_variables {
    use std::error::Error;
    use std::fs;
    use helloworld::{search, search_case_insensitive};

    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let query = args[1].clone();
            let file_path = args[2].clone();
            let ignore_case = std::env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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


}
