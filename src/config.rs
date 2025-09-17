use std::env;
use std::fs;
use std::process;

use crate::utils::print_help;

pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub dir_paths: Vec<String>,
    pub ignore_case: bool,
    pub label_files: bool,
    pub include_symlinks: bool,
    pub include_hidden: bool,
    pub include_line_numbers: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String> + std::fmt::Debug,
    ) -> Result<Config, String> {
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
        let mut include_symlinks = env::var("INCLUDE_SYMLINKS").is_ok();
        let mut include_hidden = env::var("INCLUDE_HIDDEN").is_ok();
        let mut include_line_numbers = env::var("INCLUDE_LINE_NUMBERS").is_ok();

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
                "-include_symlinks" | "s" => {
                    include_symlinks = true;
                }
                "-include_hidden" | "hi" => {
                    include_hidden = true;
                }
                "-include_line_numbers" | "l" => {
                    include_line_numbers = true;
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
            include_symlinks,
            include_hidden,
            include_line_numbers,
        })
    }
}
