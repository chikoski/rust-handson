# Rust hands-on for newbies

Mozilla Tech Speakers 

N. Shimizu (chikoski@gmail.com / [@chikoski](https://twitter.com/chikoski))

---

## Today's goal

* Convince you to think Rust is awesome :p
* Topics
   * Fundamental syntax
   * Ownership and move semantics
   * References and borrowing
* Implement simple cat and grep commands 
   * File I/O
   * Error handling with Option / Result types

---

## N.Shimizu / [@chikoski](https://twitter.com/chikoski/)

<div style="float: left; width: 14rem; margin-right: 4rem; margin-top: -2rem;">

![Avator](img/chikoski.png)
</div>

* Mozilla Tech Speakers
* Programming language enthusiast 
* Strong type system lover

---

## Programming language Rust

* Safe and efficient system programming
* Characteristics
    * Procedural language, abstract data type, closures
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

* This function sums positive values in the given list
* filter is a higher-ordered function (a closure given)

----

### Rustc emits effective machine code

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

* An assembly code emitted from sum_positive with release build

---

## Set up

* Online compiler: [http://play.rust-lang.org/](http://play.rust-lang.org/)
* Install Rust compiler to your own environment, if you want to do it offline.

----

### Compiler installation

~~~sh
% curl -sSf https://static.rust-lang.org/rustup.sh | sh
~~~

* Mac / Linux: Install with [rustup](https://www.rustup.rs/) as above
* Windows: [Installer](https://www.rust-lang.org/en-US/downloads.html) is also available

----

### Installed tools

|name|description|
|------|----|
|rustc|Rust compiler|
|cargo|Package manager|

* We will use cargo to build and run our projects

---

## "Hello, world!" with Cargo

~~~sh
% cargo new --bin hello_world
% cd hello_world
% cargo build
% cargo run
~~~

* `new` command populate project folders
* `build` command will build binary files
* `run` command will build project and run it

---

## Hello, world

<pre><code data-source="samples/helloworld/src/main.rs" data-trim class="hljs rust"></code></pre>

* [Demo](https://is.gd/w9hWRA)

----

### main function: entry point

<pre><code data-source="samples/helloworld/src/main.rs" data-trim class="hljs rust"></code></pre>

* Rust programs (not libraries) require main function as their entry point
* ```fn``` is a statement to define functions

----

### println!

<pre><code data-source="samples/helloworld/src/main.rs" data-trim class="hljs rust"></code></pre>

* A macro to output literals into consoles
* We can embed expressions

---

## Variables

<pre><code data-source="samples/bindings/src/main.rs" data-trim class="hljs rust"></code></pre>

* We can use variables only if we declare them
* ```let```: a statement to declare variables 
* [Demo](https://is.gd/rvBz9t)

----

### Variable bindings

<pre><code data-source="samples/bindings/src/main.rs" data-trim class="hljs rust"></code></pre>

* Binding: to make connection between a value and a variable name
* ```=```: operator to make variable bindings

----

### Immutable values 

<pre><code data-source="samples/re-assignment/src/main.rs" data-trim class="hljs rust"></code></pre>

* [Build error occurs when we build the this code](https://is.gd/kieuKz)
* We can not change the bound value

----

### Make another variable binding

<pre><code data-source="samples/re-binding/src/main.rs" data-trim class="hljs rust"></code></pre>

* In this case, we will not face a [build error](https://is.gd/m79jCx)
* We make another variable binding with a second ```let``` statement

----

### ```mut```: mutable variables declaration 

<pre><code data-source="samples/mutable-variable/src/main.rs" data-trim class="hljs rust"></code></pre>

* We can change binded values when their variables declared with `mut` keyword
* [Demo](https://is.gd/hx0FuE)

---

### Function declarations

<pre><code data-source="samples/function/src/main.rs" data-trim class="hljs rust"></code></pre>

* We need to put type annotations to arguments and return values
* [Demo](https://is.gd/JPFez0)

----

### Type annotation to variables

<pre><code data-source="snippets/type-annotation.rs" data-trim class="hljs rust"></code></pre>

* ```variable_name:type```
* We can omit type annotations when compiler can infer types

----

### Return values

<pre><code data-source="snippets/signature.rs" data-trim class="hljs rust"></code></pre>

* Functions return the evaluated values of their last expression
* We can also describe return values with ```return``` statement explicitely
* The return values are annotated as `-> type` 

---

## Primitive types

|Type|Description|
|---|---|
|bool|Boolean values|
|char|Unicode characters |
|i8, i16, i32, u8, u16, u32, u64, isize, usize|Integers|
|f32, f64|Floating point values|
|'static str, str, String|String values|

---

## Arrays

<pre><code data-source="snippets/array/literal.rs" data-trim class="hljs rust"></code></pre>

* Fixed length, every item belong to same type
* Annotated as ```[type; size]``` 
* We need to declare a mutable object when we want to change its items

----

### Access to each item with its index

<pre><code data-source="snippets/array/items.rs" data-trim class="hljs rust"></code></pre>

* 0 origin
* [Demo](https://is.gd/bnuUxg)

----

### len() to refer # of items 

<pre><code data-source="snippets/array/length.rs" data-trim class="hljs rust"></code></pre>

* We can refer to the arrays length by calling its ```len``` method 
* [Demo](https://is.gd/gCsQPu)

---

## Slice: reference to another structure

<pre><code data-source="snippets/array/slice.rs" data-trim class="hljs rust"></code></pre>

* Create an slice as ```&array_name[start_index..end_index]```
* [Demo](https://is.gd/TBB3nY)

---

## Tuple: fixed size ordered list

<pre><code data-source="snippets/tuple/basics.rs" data-trim class="hljs rust"></code></pre>

* We can create a fixed size ordered list as tuple
* Paris, triples, etc can be represented as tuples
* A tuple can be annotated as `(type of first element, type of second element, ...)`

----

### Access to tuple elements

<pre><code data-source="snippets/tuple/items.rs" data-trim class="hljs rust"></code></pre>

* We can access each field by destructuring
* Also each items can also be accessed with their indexes

---

## Control flow

|Control flow|Statement|
|------|--|
|Conditional branch|if, match|
|Loop|for, loop, while|

---

## Conditional branch

<pre><code data-source="snippets/if/basics.rs" data-trim class="hljs rust"></code></pre>

* We need not to write condition within parenthesis
* [Demo](https://is.gd/eA6B4C)

----

### if expression

<pre><code data-source="snippets/if/expression.rs" data-trim class="hljs rust"></code></pre>

* ```if``` is a expression, not statement in Rust
* So, it has an evaluation value
* [Demo](https://is.gd/u7ReVY)

---

## loop statement: infinite loop

<pre><code data-source="snippets/loop.rs" data-trim class="hljs rust"></code></pre>

* [Demo](https://is.gd/s7SLib)

----

## while statement

<pre><code data-source="snippets/while.rs" data-trim class="hljs rust"></code></pre>

* We do not need parenthesize loop conditions as well

---

## for statement: scan items over iterators

<pre><code data-source="snippets/iterator/for.rs" data-trim class="hljs rust"></code></pre>

* `0..10`: an object literal, which represents a list consisting of integers within the specified range

----

### enumerate method

<pre><code data-source="snippets/enumerator/for.rs" data-trim class="hljs rust"></code></pre>

* We can refer to the index value of each iteration by calling enumerate method

---

## Vector: variable length list

<pre><code data-source="snippets/vector/basics.rs" data-trim class="hljs rust"></code></pre>

* Created with ```vec!``` macro 
* For details: [Vec<T>](https://doc.rust-lang.org/stable/std/vec/)
* [Demo](https://is.gd/XDOd0T)

----

### Scaning a vector

<pre><code data-source="snippets/vector/scan.rs" data-trim class="hljs rust"></code></pre>

* Create an iterator by calling vector's ```iter``` method  
* [Demo](https://is.gd/C6dys6)

----

### filter / map

<pre><code data-source="snippets/vector/filter-map.rs" data-trim class="hljs rust"></code></pre>

* An iterator has several methods to create other iterators such as ```filter``` and ```map``` 
* [Demo](https://is.gd/fzmX9R)

---

## Exercise: fibonacci function

* [Fibonacci](https://en.wikipedia.org/wiki/fibonacci)
* [Skelton code](https://is.gd/7mKTCt) 

----

### Naive implementation with JavaScript

<pre><code data-source="snippets/fib.js" data-trim class="hljs javascript"></code></pre>

---

## Ownership and  move semantics

<pre><code data-source="snippets/ownership.rs" data-trim class="hljs javascript"></code></pre>

* This code cannot be compiled: [Demo](https://is.gd/Iy1MlJ))
* Because ownership against ```vec![1, 2, 3]``` is moved from ```v``` to ```v2```
* Variable binding means "binded value is a possesion of the biding variable" 

----

###　Copy trait

<pre><code data-source="snippets/ownership-copy-trait.rs" data-trim class="hljs rust"></code></pre>

* Some values (e.g. ```i32```) copy themselves when new variable bindings are created ([Demo](https://is.gd/iqsf3d))
    * ```x``` and ```y``` do not bind the same object
    * but, they bind different and equivalent objects
* Their type implements ```Copy``` trait 

----

### Owenership transfer via function calls

<pre><code data-source="snippets/ownership-transfer-by-function-call.rs" data-trim class="hljs rust"></code></pre>

* Causes build error ([Demo](https://is.gd/DocyHD))
* Ownership of ```vec![1, 2, 3]``` has been moved to the first argument of sum_vec, when its first call

----

### Return arguments' ownership with tuples

<pre><code data-source="snippets/ownership-restoring.rs" data-trim class="hljs rust"></code></pre>

* [Demo](https://is.gd/eSwwXR)

---

## Reference

<pre><code data-source="snippets/reference.rs" data-trim class="hljs rust"></code></pre>

* We can get a reference to a value by specifying ```&``` before its name
* References are annoted as ```&Typename```
* Functions borrow values' ownership when their reference are passed
* The ownership automatically returns when borrwing function returns

----

### Borrowed values are immutable

<pre><code data-source="snippets/reference-immutable.rs" data-trim class="hljs rust"></code></pre>

* [Demo](https://is.gd/f8vsR1)

----

### Mutable borrowed values

<pre><code data-source="snippets/reference-mutable.rs" data-trim class="hljs rust"></code></pre>

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

* Implement a function to sum all item in the given 2 vectors
* [Skeleton code](https://is.gd/HtM4t1)
* We can change the functions type annotations

---

## match expression: pattern match

<pre><code data-source="snippets/match/basic.rs" data-trim class="hljs rust"></code></pre>

* [Demo](https://is.gd/gXr0zW)
* ```_``` matches the uncovered cases

----

### `|` : union of patterns

<pre><code data-source="snippets/match/union.rs" data-trim class="hljs rust"></code></pre>

* ```|``` matches multiple patterns at once

----

### Destructuring

<pre><code data-source="snippets/match/destructuring.rs" data-trim class="hljs rust"></code></pre>

---

## Text

|Type|Description|
|--|---|
|'static str|String literal (immutable)|
|str|Slice of str / String|
|String|List of characters|

----

### Text concatenation

<pre><code data-source="snippets/string/concatenation.rs" data-trim class="hljs rust"></code></pre>

* We can create a ```String``` object from a ```str``` object by calling its ```to_string``` method
* ```fmt!``` macro is the easiest way to concatenate texts
* We can also concatenate texts with ```+``` operation   
* [Demo](https://is.gd/wFW5vf)

----

### str: how to create a slice from a static text

<pre><code data-source="snippets/str/slice.rs" data-trim class="hljs rust"></code></pre>

---

## Hands-on: cat command implementation

~~~sh
% cat -n fileA fileB
~~~

* cat is a command to output file content to standart output
* cat will output all file content continuously when we give 2 or more file names to it  
* Each line starts with its line number when ```-n``` option is specified

----

### Skeleton code generation with Cargo

~~~sh
% cargo new --bin cat
~~~

* This command will generate skeleton code for binaries, not libraries

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

<pre><code data-source="snippets/cat/arguments.rs" data-trim class="hljs rust"></code></pre>

* [```std::env::args```](https://doc.rust-lang.org/std/env/fn.args.html) returns an iterator of commmand line arguments
* ```collect``` method creates a vector from a iterator
* The first (```args[0]```) argument is the command name

----

### How to give command line options via Cargo

~~~sh
% cargo run -- src/main.rs
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/commandline-argument src/main.rs`
src/main.rs
~~~

* Program's arguments come after `--` option
* `src/main.rs` will be passed to the program in this example

----

### Reading file content

<pre><code data-source="snippets/cat/read_file.rs" data-trim class="hljs rust"></code></pre>

* ```File::open``` creates a `File` object contained in `Result` object
* `Result` object is explained in error handling section

----

### Print file's content

<pre><code data-source="snippets/cat/main.rs" data-trim class="hljs rust"></code></pre>

* Choose appropriate process according to file operation result  
    * ```Ok```: success
    * ```Err```: failed
* We can refer ```Result``` object's payload by a variable written in ```()```

----

### Challenge: add more features to your own cat

1. Change the code to accept 2 or more files
2. Implement ```-n``` option (command line options can be handled with [```getopts```](https://doc.rust-lang.org/getopts/getopts/index.html))

---

## Error handling

<pre><code data-source="samples/option/src/main.rs" data-trim class="hljs rust"></code></pre>

* We can detecting errors by checking the values of Option / Result types
* Option type: to detect values' absese
* Result type: to detect failures in some tasks

----

### Option type values: None and Some

<pre><code data-source="samples/option/src/main.rs" data-trim class="hljs rust"></code></pre>

* None: a value representing absense of expected values
* Some: a value representing existande of expected values
* We can extract values from Some by
    * Calling `unwrap` method
    * Destructuring in pattern match as above

----

### Result type values: Ok and Err

<pre><code data-source="snippets/cat/main.rs" data-trim class="hljs rust"></code></pre>

* `Ok` means success, `Err` means failure.
* We can extract values from `Ok` and `Err` by
    * Calling `unwrap` method
    * Destructuring in pattern match as above

---

## Challenge: grep command implementation

~~~sh
% grep patterns grep.txt
     match one or more patterns.  By default, a pattern matches an input line
     Each input line that matches at least one of the patterns is written to
     grep is used for simple patterns and basic regular expressions (BREs);
     grep and egrep, but can only handle fixed patterns (i.e. it does not% grep hello
~~~      

* Try to implement the grep command by modifing the cat you implemented above  
* grep is a command to pick the lines containing a specified word

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
* [Refer to this document for details](https://doc.rust-lang.org/regex/regex/index.html)

---

### Features we can add...

* Show the lines which do not match the given pattern
* Specifying patterns with regular expressions
* Speeding up with parallel programming
* etc

---

## Have fun!

* Topics we do not covered
    * User defined types（Struct / Enum）
    * Generics
    * traits, crates, modules
    * Cast
    * Life time
    * Parallelism, closure
* We need your help: [https://github.com/chikoski/rust-handson](https://github.com/chikoski/rust-handson) 
