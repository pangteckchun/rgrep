use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    // use a struct instead of tuple to make return structure meaningful from parse_config ()
    query: String,
    filename: String,
}

impl Config {
    // Constructor for Config
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage> cargo run <query string> <filename>");
        }
        // Using clone() to avoid ownership issue here; a small performance price to pay to make program more readable (for now)
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Defining Box<dyn Error> as return type of run() allows us to return a type that implements the Error trait object;
// Tthis gives us flexibility to return error values that may be of different types in different error cases
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    print!("With text:\n{}\n", contents);

    Ok(())
}
