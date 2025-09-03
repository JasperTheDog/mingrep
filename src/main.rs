use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for {} in file: {}",
        config.query, config.file_path
    );

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
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
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }
        let mut iter = args.iter();
        iter.next(); // Skip program name arg

        let query = iter.next().unwrap().clone(); // TODO, use references instead of clone.
        let file_path = iter.next().unwrap().clone();

        // env vars
        let mut ignore_case = env::var("IGNORE_CASE").is_ok(); // TODO, add both env var and arg options

        // override env vars with arguments
        let arg = iter.next();
        while !arg.is_none() {
            let flag = arg
                .expect("Cannot be None")
                .strip_prefix("-")
                .unwrap_or_else(|| {
                    println!("Problem while parsing arguments. Invalid argument. Missing '-' ");
                    process::exit(1);
                });
            match flag {
                "ignore_case" | "i" => {
                    ignore_case = true;
                }
                _ => {
                    println!("Problem when parsing arguments. Invalid argument: {}", flag);
                    process::exit(1);
                }
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
