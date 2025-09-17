use colored::Colorize;
use walkdir::DirEntry;

pub fn print_help() {
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
        "  {}, {}            Print help information",
        "-h".cyan(),
        "--help".cyan()
    );
    println!(
        "  {}, {}  Search case-insensitively. Also can set env variable IGNORE_CASE.",
        "-i".cyan(),
        "--case-insensitive".cyan()
    );
    println!(
        "  {}, {}  Header each result line with filename. Also can set env variable HEADER.",
        "-H".cyan(),
        "--header".cyan()
    );
    println!(
        "  {}, {}  Include symbolic links when recursively searching directories. Also can set env variable INCLUDE_SYMLINKS.",
        "-s".cyan(),
        "--include_symlinks".cyan()
    );
    println!(
        "  {}, {}  Include hidden files when recursively searching directories. Also can set env variable INCLUDE_HIDDEN.",
        "-hi".cyan(),
        "--include_hidden".cyan()
    );
    println!(
        "  {}, {}  Include line numbers when displaying files. Also can set env variable INCLDUE_LINE_NUMBERS.",
        "-l".cyan(),
        "--include_line_numbers".cyan()
    );
    println!(
        "{}",
        "Note: Options and files can be intermingled. Options are marked with ('-') or ('--')"
            .yellow()
    );
    println!();
    println!("{}", "Examples:".green().bold());
    println!(
        "  {} {} {} \n{}",
        "minigrep".cyan(),
        "test".yellow(),
        "poem.txt".purple(),
        "  Search for 'test' in poem.txt"
    );
    println!(
        "  {} {} {} {}",
        "minigrep".cyan(),
        "-i".green(),
        "rust".yellow(),
        "poem.txt".purple()
    );
    println!("  Search for 'rust' in poem.txt case-insensitively");
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
