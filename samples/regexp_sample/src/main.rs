extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"[Hh][Ee][Ll][Ll][Oo]").unwrap();
    let result = re.is_match("HeLLo, world"); 
    println!("{}", result);
}
