use std::error::Error;
use std::{env, fs};
use std::env::Args;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            None => return Err("Didn't get a query string"),
            Some(arg) => arg
        };
        let filename = match args.next() {
            None => return Err("Didn't get a file name"),
            Some(arg) => arg
        };

        let case_sensitive = if let Some(case_string) = args.next() {
            !case_string.to_lowercase().eq("case_insensitive")
        } else if env::var("CASE_INSENSITIVE").is_err() {
            true
        } else {
            false
        };

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tests {
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
            vec!["safe, fast, productive."],
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
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(|l| l.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
            .filter(|l| l.to_lowercase().contains(&query))
            .collect()
}
























