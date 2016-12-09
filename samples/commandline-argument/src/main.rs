fn main() {
    let mut args = std::env::args();

    println!("{}", args.len());
    if let Some(file) = args.nth(1) {
        println!("{}", file);
    }
}