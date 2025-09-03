use colored::Colorize;
use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for '{}' in file: {}",
        config.query, config.file_path
    );

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
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
    fn build(mut args: impl Iterator<Item = String> + std::fmt::Debug) -> Result<Config, String> {
        if args.any(|arg| arg == "--help" || arg == "-h") {
            print_help();
            return Err(String::from("Help option called!"));
        }
        let mut args = env::args();
        args.next(); // skip over name of process

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a query string")),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a file path")),
        };

        // env vars
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        // override env vars with arguments
        while let Some(arg) = args.next() {
            let flag = match arg.strip_prefix("-") {
                Some(f) => f,
                None => {
                    return Err(String::from("Invalid argument. Missing '-' "));
                }
            };

            match flag {
                "-ignore_case" | "i" => {
                    ignore_case = true;
                }
                _ => {
                    return Err(format!(
                        "Problem when parsing arguments. Invalid argument: {}",
                        flag
                    ));
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

fn print_help() {
    println!("Mini version of grep written in Rust by Carson Musser\n");
    println!(
        "{} {} {}",
        "Usage: ".green(),
        "minigrep".cyan().bold(),
        "[OPTIONS] <query> <filepath>".green()
    );
    println!();
    println!("{}", "Options:".green().bold());
    println!(
        "  {}, {}            Print help information",
        "-h".cyan(),
        "--help".cyan()
    );
    println!(
        "  {}, {}  Search case-insensitively. Also can set env variable IGNORE_CASE.",
        "-i".cyan(),
        "--case-insensitive".cyan()
    );
    println!();
    println!("{}", "Examples:".green().bold());
    println!(
        "  {} {} {} \n{}",
        "minigrep".cyan(),
        "test".yellow(),
        "poem.txt".purple(),
        "  Search for 'test' in poem.txt"
    );
    println!(
        "  {} {} {} {}",
        "minigrep".cyan(),
        "-i".green(),
        "rust".yellow(),
        "poem.txt".purple()
    );
    println!("  Search for 'rust' in poem.txt case-insensitively");
}
