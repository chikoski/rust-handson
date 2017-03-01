use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename: &String) -> Result<String, io::Error>{
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn main() {
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
        match read_file(file){
            Ok(content) => println!("{}", content),
            Err(reason) => panic!("{}", reason)
        }
    }
}
