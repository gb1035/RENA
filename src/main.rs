// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = Path::new("hello.txt");
    //Parse args
    if args.len() < 2
    {
        println!("hey, I need a file to parse...");
    }
    else
    {
        path = Path::new(&args[1]);
    }

    // Create a path to the desired file
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    // Creating the buffered reader
    let f = BufReader::new(file);
    // Read the file contents into a string, returns `io::Result<usize>`
    for line in f.lines(){
        let l = line.unwrap();
        println!("{}", l);
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
