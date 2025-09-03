use colored::Colorize;
use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for {} in file: {}",
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
    // Change the error type to String
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Missing arguments"));
        }
        let mut iter = args.iter();
        iter.next(); // Skip program name arg

        let query = iter.next().unwrap().clone();
        let file_path = iter.next().unwrap().clone();

        // env vars
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        // override env vars with arguments
        let mut arg = iter.next();
        while !arg.is_none() {
            let flag = match arg.unwrap().strip_prefix("-") {
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
                    // Use format! to create a dynamic String for the error
                    return Err(format!(
                        "Problem when parsing arguments. Invalid argument: {}",
                        flag
                    ));
                }
            }
            arg = iter.next();
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
