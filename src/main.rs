extern crate rustc_serialize as serialize;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::env;
use serialize::base64::FromBase64;

struct Message {
    time: f64,
    src: String,
    strdata: String,
    data: Vec<u8>,
    count: u64,
}


fn many(s: &str, cnt: u32) -> String
{
    (0..cnt).map(|_| s).collect::<String>()
}

fn print_data(data: Vec<u8>)
{
    for d in data
    {
        print!("{:02x} ", d);
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = Path::new("bendix_raw.log");
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
    let mut complete: Vec<Message> = Vec::new();
    let mut data_size = 0;
    // Read the file contents into a string, returns `io::Result<usize>`
    for line in f.lines(){
        let l = line.unwrap();
        let msg = l.split(" ").collect::<Vec<&str>>();
        let mtime:f64 = msg[0].parse().unwrap();
        let msrc = msg[1].to_string();
        let mstrdata = msg[2].to_string();
        let mdata = match mstrdata.to_string().from_base64() {
            Err(e) => panic!("decoding error {}", e),
            Ok(s) => s
        };
        if mdata.len() > data_size{data_size = mdata.len();}
        //println!("t: {} s: {} d: {}", mtime, msrc, mdata[0]);
        if complete.len() > 0 && complete.last_mut().expect("oops").strdata == mstrdata //If this is a duplicate of the last msg
        {
            complete.last_mut().expect("oops").count+=1;
        }
        else
        {
            complete.push(Message{time: mtime, src: msrc, strdata: mstrdata, data: mdata, count: 1});
        }
    }
    let data_size = data_size as u32;
    println!("{}", complete.len());
    println!("{}", many("-", 40));
    for msg in complete
    {
        print!("{:<20}", msg.time);
        print!("|");
        if msg.src != "ECM"
        {
            print_data(msg.data);
            print!("---->");
            if msg.count > 1{print!("{:^2}", msg.count);}
            else {print!("  ");}
        }
        else
        {
            //print!("{}", many(" ", data_size*3));
            if msg.count > 1{print!("{:^2}", msg.count);}
            else {print!("  ");}
            print!("<----");
            print_data(msg.data);
        }
        println!("");
    }
    println!("{}", many("-", 40));

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
