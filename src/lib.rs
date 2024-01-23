use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // automatic coercion of &Vec<String> into String slice
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { 
            return Err("not enough arguments")
        }
        let query = args[1].clone(); // for simplicity
        let file_path = args[2].clone(); // for simplicity
        Ok(Config { query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // "?" to only continue if config::build did not error
    let contents = fs::read_to_string(config.file_path)?; 
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

// result will have the same lifetime as contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_lowercase = &query.to_ascii_lowercase(); // defined outside loop, doesnt change
    for line in contents.lines() {
        let line_lowercase = &line.to_ascii_lowercase();
        if line_lowercase.contains(query_lowercase) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_result() {
        let query = "duct"; // as in pro[duct]ive
        // not indented, since indentation would appear in the string too
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "RusT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}