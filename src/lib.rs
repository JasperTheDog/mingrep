pub mod config;
pub mod utils;

use std::error::Error;
use std::fs;
use std::path::Path;

use crate::config::Config;

pub fn search<'a>(
    query: &'a str,
    contents: &'a str,
) -> Box<dyn Iterator<Item = (usize, &'a str)> + 'a> {
    Box::new(
        contents
            .lines()
            .enumerate()
            .filter(move |(_, line)| line.contains(query))
            .map(|(i, line)| (i + 1, line)),
    )
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> Box<dyn Iterator<Item = (usize, &'a str)> + 'a> {
    Box::new(
        contents
            .lines()
            .enumerate()
            .filter(move |(_, line)| line.to_lowercase().contains(&query.to_lowercase()))
            .map(|(i, line)| (i + 1, line)),
    )
}

pub fn search_file(file_path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for (line_number, line_content) in results {
        let mut prefix = String::new();

        if config.label_files {
            prefix.push_str(&format!("{}:", file_path.display()));
        }
        if config.include_line_numbers {
            prefix.push_str(&format!("{}:", line_number));
        }

        println!("{}{}", prefix, line_content);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(query, contents)
        );
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
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_case_insensitive(query, contents)
        );
    }
}
