use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    let mut f = File::open(filename).expect("file not found"); // panic if file not found with custom message

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file"); // panic if file can't read for whatever reasons, with custom message

    print!("With text:\n{}", contents);
}
