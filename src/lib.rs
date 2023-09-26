use std::error::Error;
use std::{env, fs};

pub struct Config {
    query: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(s) => s,
            None => return Err(""),
        };
        let filename = match args.next() {
            Some(s) => s,
            None => return Err(""),
        };
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.is_case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &'_ str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

mod tests {
    use crate::{search, search_case_insensitive};

    const TEXT: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, TEXT));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "du";
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, TEXT)
        );
    }
}
