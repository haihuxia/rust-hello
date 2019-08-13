use std::error::Error;
use std::fs;
use std::env;

pub mod slices;
pub mod structs;
pub mod enums;
pub mod matchs;
pub mod iflet;
pub mod modules;
pub mod vectors;
pub mod strings;
pub mod hashmaps;
pub mod errors;
pub mod gennic;
pub mod traits;
pub mod lifetimes;
pub mod iterators;
pub mod boxpoints;
pub mod deref;
pub mod drop;
pub mod rc;
pub mod refcell;
pub mod weak;
pub mod thread;
pub mod channel;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new12(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // CASE_INSENSITIVE=1 cargo run to poem.txt
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }

    pub fn new13(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // CASE_INSENSITIVE=1 cargo run to poem.txt
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }

    fn new_a(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // CASE_INSENSITIVE=1 cargo run to poem.txt
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Config { query, filename, case_sensitive }
    }
}

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

fn parse_config_a(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_a<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim_left());
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim_left());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::search;
    use crate::search_case_insensitive;

    #[test]
    fn one_work() {
        let query = "duct";
        let contents = "\
                Rust:
                safe, fast, productive
                Pick three.";

        assert_eq!(vec!["                safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
                Rust:
                safe, fast, productive
                Pick three.
                Duct tape.";

        assert_eq!(vec!["                safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
                Rust:
                safe, fast, productive
                Pick three.
                Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}