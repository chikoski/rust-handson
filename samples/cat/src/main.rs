extern crate getopts;

use std::env;
use getopts::Options;

fn start(files: &Vec<String>, flag_n:bool){
    for filename in files{
        println!("{}", filename);
    }
}

fn print_usage(program: &str, options:Options){
    let brief = format!("{} [-n] file [file..]", program);
    print!("{}", options.usage(&brief));
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

    start(&options.free, flag_n);
}
