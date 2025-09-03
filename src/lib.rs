pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_with_query = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            lines_with_query.push(line);
        }
    }
    lines_with_query
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut lines_with_query = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            lines_with_query.push(line);
        }
    }
    lines_with_query
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
