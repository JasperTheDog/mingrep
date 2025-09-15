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
        "Searching for '{}' in file(s): {:?} and in dir(s): {:?}",
        config.query, config.file_paths, config.dir_paths
    );

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file_path in config.file_paths {
        let contents = fs::read_to_string(&file_path)?;

        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            if config.label_files {
                println!("{file_path}:{line}");
            } else {
                println!("{line}");
            }
        }
    }

    Ok(())
}
struct Config {
    query: String,
    file_paths: Vec<String>,
    dir_paths: Vec<String>,
    ignore_case: bool,
    label_files: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String> + std::fmt::Debug) -> Result<Config, String> {
        if args.any(|arg| arg == "--help" || arg == "-h") {
            print_help();
            process::exit(0);
        }
        let mut args = env::args();
        args.next(); // skip over name of process

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a query string")),
        };

        let mut file_paths = vec![];
        let mut dir_paths = vec![];

        // env vars
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        let mut label_files = env::var("HEADER").is_ok();

        // override env vars with arguments and parse file_paths
        while let Some(arg) = args.next() {
            let flag = match arg.strip_prefix("-") {
                Some(f) => f,
                None => {
                    let metadata = match fs::metadata(&arg) {
                        Ok(metadata) => metadata,
                        Err(e) => {
                            return Err(format!("Error parsing metadata on path '{}': {}", arg, e));
                        }
                    };
                    if metadata.is_dir() {
                        dir_paths.push(arg);
                    } else if metadata.is_file() {
                        file_paths.push(arg);
                    }
                    continue;
                }
            };

            match flag {
                "-ignore_case" | "i" => {
                    ignore_case = true;
                }
                "-header" | "H" => {
                    label_files = true;
                }
                _ => {
                    return Err(format!(
                        "Problem when parsing arguments. Invalid argument: -{}",
                        flag
                    ));
                }
            }
        }
        Ok(Config {
            query,
            file_paths,
            dir_paths,
            ignore_case,
            label_files,
        })
    }
}

fn print_help() {
    println!("Mini version of grep written in Rust by Carson Musser\n");
    println!(
        "{} {} {}",
        "Usage: ".green(),
        "minigrep".cyan().bold(),
        "<query> [FILE ...] [OPTIONS]".green()
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
    println!(
        "  {}, {}  Header each result line with filename. Also can set env variable HEADER.",
        "-H".cyan(),
        "--header".cyan()
    );
    println!(
        "{}",
        "Note: Options and files can be intermingled. Options are marked with ('-') or ('--')"
            .yellow()
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
