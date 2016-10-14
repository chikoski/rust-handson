use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename:String) -> Result<String, io::Error>{
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() > 1{
        println!("{}", match read_file(args[1].clone()){
            Ok(content) => content,
            Err(reason) => panic!(reason) 
        });
    }
}
