fn main() {
    let mut args = std::env::args();
    match args.nth(1) {
        Some(filename) => println!("{} is specified", filename),
        None => println!("No file name is specified"),
    };
}