extern crate getopts;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use getopts::Options;

fn run(filename:String) -> Result<(String),io::Error>{
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn with_line_number(content:String) -> String{
    let lines: Vec<String> = 
        content.split("\n")
                .filter(|line| line.len() > 0)
                .enumerate()
                .map(|(index, line)| 
                        format!("{}: {}", index + 1, line))
                .collect();
    lines.join("\n")
}

fn start(files: Vec<String>, flag_n:bool){
    for filename in files{
        match run(filename){
            Err(reason) => panic!(reason),
            Ok(content) => print!("{}", if flag_n {
                with_line_number(content)
            }else{
                content
            })
        }
    }
}

fn print_usage(program: &str, options:Options){
    let brief = format!("{} [-n] file [file..]", program);
    println!("{}", options.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parser = Options::new();
    let program = args[0].clone();

    parser.optflag("n", "", "Number the output lines, starting at 1.");

    let options = match parser.parse(&args[1..]){
        Ok(m) => {m}
        Err(f) => { panic!(f.to_string())}
    };

    let flag_n = options.opt_present("n");

    if options.free.is_empty(){
        print_usage(&program, parser);
        return;
    }

    start(options.free, flag_n);
}
