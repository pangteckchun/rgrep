use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // unwrap_or_else() is a Result<T, E> method that takes a closure as argument; allows us to define custom, non-panic error handling
    // if Result is Ok, unwrap_or_else() returns the value inside the Ok
    // if Result is Err, unwrap_or_else() calls the closure we passed to it, passing the Err value as an argument to the closure
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage> cargo run <query string> <filename>");
        }
        // Using clone() to avoid ownership issue here; a small performance price to pay to make program more readable (for now)
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
