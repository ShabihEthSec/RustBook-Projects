use std::env;
use std::fs;
use std::error::Error;


pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Provide more arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query, file_path, ignore_case,})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for lines in results {
        println!("{lines}")
    }
    Ok(())
}

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
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}



#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
fast, performance, productivity.
Pick three.";

        assert_eq!(vec!["fast, performance, productivity."], search(&query, &contents));
    }

    #[test]
    fn test_search_case_insensitive() {
           let query = "rUsT";
        let contents = "\
Rust:
fast, performance, productivity.
Pick three.
Trust.";

        assert_eq!(vec!["Rust:", "Trust."], search_case_insensitive(&query, &contents));
    }
}
