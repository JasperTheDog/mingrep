use std::env;
use std::error::Error;
use std::fs;
use std::process;
use walkdir::WalkDir;

use minigrep::config::Config;
use minigrep::search_file;
use minigrep::utils::is_hidden;

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
    for file_path in &config.file_paths {
        search_file(file_path.as_ref(), &config)?;
    }

    for dir in &config.dir_paths {
        let mut walker = WalkDir::new(dir);
        if config.include_symlinks {
            walker = walker.follow_links(true);
        }

        let iter = walker.into_iter().filter_map(|e| e.ok());

        for entry in iter.filter(|e| !config.include_hidden || !is_hidden(e)) {
            let path = entry.path();

            if entry.file_type().is_symlink() {
                let target_path = fs::read_link(path)?;

                if target_path.is_file() {
                    search_file(&target_path, &config)?;
                }
            } else if entry.file_type().is_file() {
                search_file(path, &config)?;
            }
        }
    }

    Ok(())
}
