use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);

    let mut f = File::open(config.filename).expect("file not found"); // panic if file not found with custom message

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file"); // panic if file can't read for whatever reasons, with custom message

    print!("With text:\n{}", contents);
}

struct Config {
    // use a struct instead of tuple to make return structure meaningful from parse_config ()
    query: String,
    filename: String,
}

impl Config {
    // Constructor for Config
    fn new(args: &[String]) -> Config {
        // Using clone() to avoid ownership issue here; a small performance price to pay to make program more readable (for now)
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
