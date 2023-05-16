extern crate rgrep;

use std::env;
use std::process;

use rgrep::Config;

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

    if let Err(e) = rgrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1); 
    }
}
