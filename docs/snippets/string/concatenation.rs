let hello = "Hello"; 
let world = "world!";
let hello = hello.to_string();
let mut buffer = new String();
buffer = buffer + &hello + world;
println!("{}", buffer);