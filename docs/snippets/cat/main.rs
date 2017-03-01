fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match read_file(args[1].clone()) {
            Ok(content) => printfln!("{}", content),
            Err(reason) => panic!(reason) 
        });
    }
}