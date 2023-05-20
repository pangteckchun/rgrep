use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    // use a struct instead of tuple to make return structure meaningful from parse_config ()
    query: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    // Constructor for Config
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage> cargo run <query string> <filename> [-c == case sensitive, optional]");
        }
        // Using clone() to avoid ownership issue here; a small performance price to pay to make program more readable (for now)
        let query = args[1].clone();
        let filename = args[2].clone();
        let mut is_case_sensitive = false;

        if args.len() == 4 {
            if args[3].eq_ignore_ascii_case("-c") {
                is_case_sensitive = true;
            }
        }

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}

// Defining Box<dyn Error> as return type of run() allows us to return a type that implements the Error trait object;
// Tthis gives us flexibility to return error values that may be of different types in different error cases
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    if config.is_case_sensitive {
        for line in search(&config.query, &contents) {
            println!("Found: {}", line);
        }
    } else {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("Found: {}", line);
        }
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
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape";
        // Strings cannot be indented, else the assert will fail

        println!("Contents prep: {}", contents); // passed tests does not println!() !!! Failed one does!

        assert_eq!(vec!["Safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust me.";
        // Strings cannot be indented, else the assert will fail

        println!("Contents prep: {}", contents); // passed tests does not println!() !!! Failed one does!

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
