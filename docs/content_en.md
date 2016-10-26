# Rust hands-on for newbies

Mozilla Japan 

N. Shimizu (nshimizu@mozilla-japan.org)

---

## Today's goal

* Convince you to think Rust is awesome :p
* Topics
   * Fundamental syntax
   * Owenership and move semantics
   * Reference and borrowing
* Implement simple cat and grep command  

---

## N.Shimizu / [@chikoski](https://twitter.com/chikoski/)

<div style="float: left; width: 14rem; margin-right: 4rem; margin-top: -2rem;">

![Avator](img/chikoski.png)
</div>

* Dev-rel in Mozilla Japan
* Programming language enthusiast 
* Strong type system lover

---

## Programming language Rust

* Safe and efficient system programming
* Characteristics
    * Procedural language, abstract data type, closure
    * Static typing, strong type inference
    * Safe pointer operations
    * Parallel programming with message passing / shared memory

----

### Well-balanced design: speed & safety

~~~rust
fn sum_pos(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v.iter().filter(|i| **i > 0) {
        sum += *i;
    }
    sum
}
~~~

* This function sums positive values in given list
* filter is a high-ordered function (a closure given)

----

### Rustc emits effectve machine code

~~~x86asm
    leaq    (%rdi,%rsi,4), %rcx
    xorl    %eax, %eax
    jmp .LBB5_1
.LBB5_3:
    addl    %edx, %eax
    .align  16, 0x90
.LBB5_1:
    cmpq    %rdi, %rcx
    je  .LBB5_4
    movl    (%rdi), %edx
    addq    $4, %rdi
    testl   %edx, %edx
    jle .LBB5_1
    jmp .LBB5_3
.LBB5_4:
    retq
~~~

* An assembly code emmtied from sum_positive with release build

---

## Set up

* Online compiler: [http://play.rust-lang.org/](http://play.rust-lang.org/)
* Install Rust compiler to your own environment, if you want to do at offline.

----

### Compiler installation

~~~sh
% curl -sSf https://static.rust-lang.org/rustup.sh | sh
~~~

* Mac / Linux: Install with [rustup](https://www.rustup.rs/)
* Windows: Install with [installer](https://www.rust-lang.org/en-US/downloads.html)

----

### Installed tools

|name|description|
|------|----|
|rustc|Rust compiler|
|cargo|Package manager|

* We will use cargo to build and run our projects

---

## Hello World

~~~rust
fn main() {
    println!("Hello, world!");
}
~~~

* [Demo](https://is.gd/w9hWRA)

----

### main function: entry point

~~~rust
fn main() {
    println!("Hello, world!");
}
~~~

* Rust programs (not libraries) require main function as their entry point
* ```fn``` is a statement to define functions

----

### println!

~~~rust
fn main() {
    println!("Hello, world!");
}
~~~

* A macro to output literals into consoles
* We can embed expressions

---

## Variables

~~~rust
fn main() {
    let name = "Chiko";
    println!("Hello, {}", name);
}
~~~

* We can use variables only if we declare them
* ```let```: a statement to declare variables 
* [Demo](https://is.gd/rvBz9t)

----

### Variable bindings

~~~rust
fn main() {
    let name = "Chiko";
    println!("Hello, {}", name);
}
~~~

* Binding: to make connection between a value and a varible name
* ```=```: operator to make variable bindings

----

### Immutable values 

~~~rust
fn main() {
    let name = "Chiko";
    println!("Hello, {}", name);
    name = "Shimizu";
    println!("Hello, {}", name);
}
~~~

* [Build error occurs when we build the this code](https://is.gd/kieuKz)
* We can not change binded values

----

### Make another variable binding

~~~rust
fn main() {
    let name = "Chiko";
    println!("Hello, {}", name);
    let name = "Shimizu";
    println!("Hello, {}", name);
}
~~~

* In this case, we will not face [build error](https://is.gd/m79jCx)
* We make another variable binding with second ```let``` statement

----

### ```mut```: mutable variables declaration 

~~~rust
fn main() {
    let mut name = "Chiko";
    println!("Hello, {}", name);
    name = "Shimizu";
    println!("Hello, {}", name);
}
~~~

* We can change binded values when their variables declared with `mut` keyword
* [Demo](https://is.gd/hx0FuE)

---

### Function declarations

~~~rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("1 + 1 = {}", add(1, 1));
    println!("13 + 23 = {}", add(13, 23));
}
~~~

* We need to put type annotations to arguments and return value
* [Demo](https://is.gd/JPFez0)

----

### Type annotation to variables

~~~rust
fn add(a: i32, b: i32) -> i32 {
    let answer: i32;
    answer = a + b;
    answer
}
~~~

* ```variable_name:type```
* We can ommit type annotations when compiler can infer types

----

### Return values

~~~rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn another_add(a: i32, b: i32) -> i32 {
    return a + b;
}
~~~

* Functions returns the evaluated values of their last expression
* We can also describe return values with ```return``` statement expricitly
* The return values are annotated as `-> type` 

---

## Primitive types

|Type|Description|
|---|---|
|bool|Boolean values|
|char|Unicode characters |
|i8, i16, i32, u8, u16, u32, u64, isize, usize|Integers|
|f32, f64|Floating point values|

---

## Arrays

~~~rust
let a = [1, 2, 3]; // a: [i32; 3]
let mut m = [1, 2, 3]; // m: [i32; 3]
~~~

* Fixed length, every items belong to same type
* Annotated as ```[type; size]``` 
* We need to declare as mutable object when we want to chanage its items

----

### Access to each item with its index

~~~rust
let a = [1, 2, 3];
println!("0th element: {}", a[0]);
println!("1st element: {}", a[1]);
println!("2nd element: {}", a[2]);
~~~

* 0 origin
* [Demo](https://is.gd/bnuUxg)

----

### len() to refer # of items 

~~~rust
let a = [1, 2, 3];
println!("size of array a: {}", a.len());
~~~

* We can refer the array length by calling its ```len``` method 
* [Demo](https://is.gd/gCsQPu)

---

## Slice: reference to another structure

~~~rust
let a = [0, 1, 2, 3, 4, 5];
let middle = &a[1..4];
println!("a.len() = {}", a.len());
println!("middle.len() = {}", middle.len());
~~~

* Create an slice as ```&array_name[start_index..end_index]```
* [Demo](https://is.gd/TBB3nY)

---

## Tuple: fixed size ordered list

~~~rust
let x = (1, "hello"); // x: (i32, &str)
let mut p1 = (1, 2); // p1: (i32, i32)
let p2 = (3, 4); // p2: (i32, i32)
p1 = p2;
~~~

* We can create a fixed size ordered list as tuple
* Paris, triples, etc can be represented as tuples
* A tuple can be annotated as `(type of first element, type of second element, ...)`

----

### Access to tuple elements

~~~rust
let p = (1, 2, 3);
let (x, y, z) = p;
let x = p.0;
let y = p.1;
let z = p.2;
~~~

* We can access each field by desctructuring
* Also accessible each items with their indexes

---

## Control flow

|Conrol flow|Statement|
|------|--|
|Conditional branch|if, match|
|Loop|for, loop, while|

---

## Conditional branch

~~~rust
let x = 5;
if x > 10 {
    println!("x > 10")
} else if x > 0 {
    println!("x < x <= 10")
} else {
    println!("x < 0")
};
~~~

* We need not to write condition within parenthesis
* [Demo](https://is.gd/eA6B4C)

----

### if expression

~~~rust
let x = 5;
let y = if x > 0 {
    1
} else {
    0
};
println!("y = {}", y);   
~~~

* ```if``` is a expresison, not statement, in Rust 
* So, it has an evaluation value
* [Demo](https://is.gd/u7ReVY)

---

## loop statement: infinite loop

~~~rust
let mut i = 0;
loop {
    i = i +1;
    println!("{} times output", i);
    if i > 9 {
       break;
    }
}
~~~

* [Demo](https://is.gd/s7SLib)

----

## while statement

~~~rust
let mut x = 5; // mut x: i32
let mut done = false; // mut done: bool

while !done {
    x += x - 3;
    println!("{}", x);
    if x % 5 == 0 {
        done = true;
    }
}
~~~

* We do not need parenthesize loop conditions as well

---

## for statement: scan items over iterators

~~~ rust
for x in 0..10 {
    println!("{}", x); // x: i32
}
~~~

* `0..10`: an object literal, which represents a list consists of integers within the specified ragnge

----

### enumerate method

~~~ rust
for (index, value) in (0..10).enumerate() {
    println!("index = {}, value = {}", index, value);
}
~~~

* We can refer the index value of each iteration by calling enumerate method

---

## Vector: variable length list

~~~rust
let v = vec![1, 2, 3, 4, 5];
let zeroes = vec![0; 10];
for i in 0..v.len() {
    println!("v[{}] = {}", i, v[i]);
}
~~~

* Created with ```vec!``` macro 
* For details: [Vec<T>](https://doc.rust-lang.org/stable/std/vec/)
* [Demo](https://is.gd/XDOd0T)

----

### Scaning a vector

~~~rust
let v = vec![1, 2, 3, 4, 5];
for (index, value) in v.iter().enumerate() {
    println!("v[{}] = {}", index, value);
}
~~~

* Create an iterator by calling vector's ```iter``` method  
* [Demo](https://is.gd/C6dys6)

----

### filter / map

~~~rust
let v = vec![1, 2, 3, 4, 5];
let result = v.iter().filter(|&n| n % 2 != 0).map(|n| n + 1);
for (index, value) in result.enumerate() {
    println!("result[{}]:{}", index, value);
}
~~~

* An iterator has several methods to create another iterators such as ```filter``` and ```map``` 
* [Demo](https://is.gd/fzmX9R)

---

## Exercise: fibonacci function

* [Fibonacci](https://en.wikipedia.org/wiki/fibonacci)
* [Skelton code](https://is.gd/7mKTCt) 

----

### Naive implmentation with JavaScript

~~~javascript
function fib(n){
  if(n == 0){
    return 0;
  }else if(n == 1){
    return 1;
  }else{
    return fib(n - 1) + fib(n - 2);
  }
}
~~~

---

## Ownership and  move semantics

~~~rust
let v = vec![1, 2, 3];
println!("v[1] = {}", v[1]);
let v2 = v;
println!("v2[1] = {}", v2[1]);
println!("v[1] = {}", v[1]);
~~~

* This code can not be compiled: [Demo](https://is.gd/Iy1MlJ))
* Because ownership against ```vec![1, 2, 3]``` is moved from ```v``` to ```v2```
* Variable binding means "binded value is a possesion of the biding variable" 

----

###　Copy trait

~~~rust
let x = 10;
let y = x;
println!("x = {}", x);
~~~

* Some values (e.g. ```i32```) copy themselves when new variable binding created ([Demo](https://is.gd/iqsf3d))
    * ```x``` and ```y``` do not bind same object
    * but, they bind different and equivalent object
* Their type imlements ```Copy``` trait 

----

### Owenership transfer via function calls

~~~rust
fn sum_vec(memo:Vec<i32>)->i32{}
fn main() {
    let v = vec![1, 2, 3];
    println!("sum = {}",sum_vec(v));
    println!("sum = {}",sum_vec(v));    
}  
~~~

* Causes build error ([Demo](https://is.gd/DocyHD))
* Ownership of ```vec![1, 2, 3]``` has been moved to the first argument of sum_vec, when its first call

----

### Return arguments' ownership with tuples

~~~rust
fn sum_vec(memo:Vec<i32>)->(i32, Vec<i32>){}
fn main() {
    let v = vec![1, 2, 3];
    let (sum, v) = sum_vec(v);
    println!("sum = {}", sum);   
    let (sum, v) = sum_vec(v);
    println!("sum = {}", sum);
}
~~~

* [Demo](https://is.gd/eSwwXR)

---

## Reference

~~~rust
fn sum_vec(memo: &Vec<i32>)->i32{}
fn main() {
    let v = vec![1, 2, 3];
    println!("sum = {}", sum_vec(&v));
    println!("sum = {}", sum_vec(&v));    
}
~~~

* We can get a reference to a value by specifying ```&``` before its name
* The variables are annoted as ```&Typename```, which are binded to a value belonging to the type
* We can borrow the ownership of a value to functions temporaly by passing its reference 
* The ownership automatically returns to the owners when the function call finished

----

### Borrwed values are immutable

~~~rust
fn foo(v: &Vec<i32>){
    v.push(5);
}
fn main(){
    let v = vec![];
    foo(&v);
}    
~~~

* [Demo](https://is.gd/f8vsR1)

----

### Mutable borrwed values

~~~rust
fn foo(v: &mut Vec<i32>){
    v.push(5);
}
fn main(){
    let mut v = vec![];
    println!("v.len() = {}", v.len());
    foo(&mut v);
    println!("v.len() = {}", v.len());    
} 
~~~

* We can get mutable references with ```&mut``` 
* Only if the values are mutable 

----

### Rules of borrowing

* We can create one ore more read only references (```&```) 
* The # of mutable reference is limited to 1 at most（```&mut```）
    * Only 1 thread can be change the state
    * This limitation is set to avoid race conditions

---

## Exercise: implemnet funciton foo 

~~~rust
fn foo(v1:Vec<i32>, v2:Vec<i32>){
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(v1, v2);
    println!("{}", answer);   
}
~~~

* Implment a function to sum all item in the given 2 vectors
* [Skelton code](https://is.gd/HtM4t1)
* We can change the funcitons' type annotations

---

## match expression: patten match

~~~rust
let x = 5;
let label = match x {
    1 => "one",
    2 => "two",
    3 => "three",
    _ => "many"
};
println!("{} : {}", x, label);
~~~

* [Demo](https://is.gd/gXr0zW)
* ```_``` matches the 


----

### 複数のパターン

~~~rust
let x = 5;
let label = match x {
    1 => "one",
    2 => "tow",
    3 => "three",
    4 | 5 | 6 | 7 | 8 | 9 => "< 10",
    _ => "many",
};
println!("{} : {}", x, label);
~~~

* We can combine two or more patterns with ```|``` 

----

### Destructuring

~~~rust
let x = (1, 5);
let label = match x {
    (0, 0) => "Zero vector",
    (1, 0) | (0, 1) => "Unit vector",
    (1, _) => "(1, ?)",
    (_, 1) => "(?, 1)",
    _ => "The others"
};
println!("{}", label);
~~~

---

## Texts

|Type|Description|
|--|---|
|'static str|String literal (immutable)|
|str|Slice of str / String|
|String|List of characters|

----

### Text concatination

~~~rust
let hello = "Hello"; 
let world = "world!";
let hello_world = fmt!("{} {}", hello, world);

let mut buffer = new String();
buffer = buffer + &(hello.toStirng()) + world;
println!("{}", buffer);
~~~

* We can create a ```String``` object from a ```str``` object by calling its ```to_string``` method
* ```fmt!``` macro is the easiest way to concatinate texts
* We can also concatintate texts with ```+``` operation   
* [Demo](https://is.gd/wFW5vf)

----

### str: how to create a slice from a static text

~~~rust
let dog = "hachiko";
let hachi = &dog[0..5];
~~~

---

## Hands-on: cat command implementation

~~~sh
% cat -n fileA fileB
~~~

* cat is a command to output file content to standart output
* cat will output all file content continuously when we give 2 or more file names to it  
* Each line starts with its line number when ```-n``` option is specified

----

### Skelton code generation with Cargo

~~~sh
% cargo new --bin cat
~~~

* This command will generate skelton code for binaries, not libraries

----

### Build and run

~~~sh
% cd cat
% cargo run
   Compiling cat v0.1.0
    Finished debug [unoptimized + debuginfo] target(s) in 1.45 secs
     Running `target/debug/helloworld`
Hello, world!
~~~

* ```cargo run``` to build the code and run it 
* ```build```  

----

### Command line arguments

~~~rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("{}", args[1]);
    }
}
~~~

* [```std::env::args```](https://doc.rust-lang.org/std/env/fn.args.html) returns an iterator containing commmand line arguments
* ```collect``` method returns a vector object consists of elemnent in the iterator
* The first (```args[0]```) argument is the command name

----

### Reading file content

~~~rust
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename: String) -> Result<String, io::Error> {
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}
~~~

* We can access file content via ```File``` object, which is created by calling ```File::open```
* ```File::open``` returns ```Result``` object, with which we can determin the file operation failed or not

----

### Print file's content

~~~rust
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("{}", match read_file(args[1].clone()) {
            Ok(content) => content,
            Err(reason) => panic!(reason) 
        });
    }
}
~~~

* Changes the control flow accoding to the file operation's result with a pattern match  
    * ```Ok```: success
    * ```Err```: failed
* We can refer ```Result``` object's payload by a variable written in ```()```

----

### Challenge: add more features to your own cat

1. Change the code to accept 2 or more files
2. Implement ```-n``` option (command line options can be handled with [```getopts```](https://doc.rust-lang.org/getopts/getopts/index.html))

---

## Challange: grep command implementation

~~~sh
% grep patterns grep.txt
     match one or more patterns.  By default, a pattern matches an input line
     Each input line that matches at least one of the patterns is written to
     grep is used for simple patterns and basic regular expressions (BREs);
     grep and egrep, but can only handle fixed patterns (i.e. it does not% grep hello
~~~      

* Try to implement grep command by modifing the cat you implemented above  
* grep is a command to pick the lines containing specified word

----

### Add regex to dependencies list

~~~
[dependencies]
regex = "0.1"
~~~

* Rust's standard library does not provide regular expression features
* regex: a regular expression library
* We can install regex by put it into Cargo.toml

----

###　regex::Regex : Regular expression object

~~~rust
extern crate regex;
use regex::Regex;

//snipped

let re = Regex::new(r"[Hh][Ee][Ll][Ll][Oo]").unwrap();
re.is_match("HeLLo, world"); // true
~~~

* We can test a text match the regular expression by calling its ```is_match``` method
* [Refer this document for details](https://doc.rust-lang.org/regex/regex/index.html)

---

### Features we can add...

* Show the lines which do not match given pattern
* Specifying patterns with regular expressions
* Speeding up with parallel programming
* etc

---

## Have fun!

* Topics we do not covered
    * User defined types（Struct / Enum）
    * Generics
    * traits, creates, modules
    * Cast
    * Life time
    * Parallelism, closure
* We need your help: [https://github.com/chikoski/rust-handson](https://github.com/chikoski/rust-handson) 
