# Rust 初心者向けハンズオン

Mozilla Japan 

清水智公 (nshimizu@mozilla-japan.org)

---

## 今日のゴール

* Rust っていい言語だなーと思ってもらうこと
* トピック
   * 基本的な構文
   * owenership と borrowing
   * ユーザー定義型（struct, enum, trait）
   * 並列処理（メッセージパッシングと共有メモリー）
* 最終的には簡単な [cat コマンド](http://itpro.nikkeibp.co.jp/article/COLUMN/20060227/230725/)と、同じく簡単な [grep コマンド](http://itpro.nikkeibp.co.jp/article/COLUMN/20060227/230786/)を実装します     

---

## 清水智公 / [@chikoski](https://twitter.com/chikoski/)

<div style="float: left; width: 14rem; margin-right: 4rem; margin-top: -2rem;">

![清水智公のアバター](img/chikoski.png)
</div>

* Mozilla Japan で Dev-rel やっています
* プログラミング言語とサッカーが好きです 
* [html5experts.jp](https://html5experts.jp/chikoski/) で記事をいくつか書いています
* html5j Web プラットフォーム部 / ゲーム部
* 型は強い方が好きです

---

## Rust ってどういう言語？

* 安全で、効率的なシステムプログラミングを！
* 特徴
    * 手続き、抽象データ型、クロージャー
    * 静的型づけ、型推論
    * 安全なポインター操作（ownership、move semantics、borrowing）
    * 並列計算（メッセージパッシング、共有メモリ）

----

### 速度と生産性のバランスがよい！

~~~rust
fn sum_pos(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v.iter().filter(|i| **i > 0) {
        sum += *i;
    }
    sum
}
~~~

* リスト v の要素の中で、正の整数の総和を返す関数
* filter に着目

----

### ネイティブコードが出力できます

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

* sum_positive から生成される x86_64 アセンブラ
* リースビルドした結果    

----

### 並列計算

---

## 環境設定

* [http://play.rust-lang.org/](http://play.rust-lang.org/) を利用します
* オフラインで実行するには、コンパイラのインストールが必要です

----

### コンパイラーのインストール

~~~sh
% curl -sSf https://static.rust-lang.org/rustup.sh | sh
~~~

* Mac / Linux の場合： [rustup](https://www.rustup.rs/) を利用してインストールします
* Windows の場合：[インストーラーをダウンロード](https://www.rust-lang.org/en-US/downloads.html)して、インストールします

----

### インストールされるツール

|ツール名|説明|
|------|----|
|rustc|Rust のコンパイラー|
|cargo|パッケージ管理ツール|

* ビルドや実行は cargo を使って行います

---

## Hello World

~~~rust
fn main(){
    println!("Hello, world!");
}
~~~

* [こちらで実行できます](https://is.gd/w9hWRA)
* 一から書きたい場合は [こちら](https://is.gd/LR2nNa)

----

### main 関数

~~~rust
fn main(){
    println!("Hello, world!");
}
~~~

* Rust のプログラムは、main 関数から実行が始まります
* fn は関数を定義する文です

----

### println!

~~~rust
fn main(){
    println!("Hello, world!");
}
~~~

* リテラルを標準出力へ出力するマクロ
* 変数の埋め込みにも対応

---

## 変数

~~~rust
fn main(){
    let name = "Chiko";
    println!("Hello, {}", name);
}
~~~

* 変数は宣言してから利用します（束縛 / 参照）
* let 文で変数名を宣言できます
* [デモ](https://is.gd/rvBz9t)

----

### 変数の束縛

~~~rust
fn main(){
    let name = "Chiko";
    println!("Hello, {}", name);
}
~~~

* 束縛：値と名前を結びつけること
* 便宜上、代入と呼ぶこともあります

----

### 再代入は原則できません

~~~rust
fn main(){
    let name = "Chiko";
    println!("Hello, {}", name);
    name = "Shimizu";
    println!("Hello, {}", name);
}
~~~

* [コンパイルエラー](https://is.gd/kieuKz)になります
* let 文で宣言された変数は変更できません

----

### 再び束縛する

~~~rust
fn main(){
    let name = "Chiko";
    println!("Hello, {}", name);
    let name = "Shimizu";
    println!("Hello, {}", name);
}
~~~

* これは[コンパイルエラーが起きません](https://is.gd/m79jCx)
* 束縛を作り直しています（代入ではないことに注意）

----

### 変更可能な変数

~~~rust
fn main(){
    let mut name = "Chiko";
    println!("Hello, {}", name);
    name = "Shimizu";
    println!("Hello, {}", name);   
}
~~~

* mut 修飾子をつけると、変更可能な変数が宣言されます
* [デモページ](https://is.gd/hx0FuE)

---

### 関数宣言

~~~rust
fn add(a:i32, b:i32) -> i32{
    a + b
}

fn main(){
    println!("1 + 1 = {}", add(1, 1));
    println!("13 + 23 = {}", add(13, 23));   
}
~~~

* 引数と返り値の型を明示します
* [動作例](https://is.gd/JPFez0)

----

### 変数に対する型アノテーション

~~~rust
fn add(a:i32, b:i32) -> i32{
    let answer: i32;
    answer =  a + b;
    answer
}
~~~

* ```変数:型``` と書くことで変数の型を明示できます
* 型が推論できる場合は、型アノテーションは省略できます

----

### 返り値

~~~rust
fn add(a:i32, b:i32) -> i32{
    a + b
}
fn another_add(a:i32, b:i32) -> i32{
    return a + b;
}
~~~

* 最後の式の評価値が返り値となります
* return 文で明示することもできます
* 返り値の型は `-> 型` とアノテーションされます

---

## 基本型

|型名|説明|
|---|---|
|bool|ブール値|
|char|文字型|
|i8, i16, i32, u8, u16, u32, u64, isize, usize|整数型|
|f32, f64|浮動小数点型|

---

## 配列

~~~rust
let a = [1, 2, 3]; // a: [i32; 3]
let mut m = [1, 2, 3]; // m: [i32; 3]
~~~

* 固定長で、要素は全て同じデータ型です
* ```[型名; 要素数]``` と型アノテーションします
* 要素を変更するためには mut 修飾子が必要です

----

### 要素の参照

~~~rust
let a = [1, 2, 3];
println!("0番目の要素: {}", a[0]);
println!("1番目の要素: {}", a[1]);
println!("2番目の要素: {}", a[2]);
~~~

* 添え字で、個々の要素へアクセスします
* 先頭の添え字は 0 です
* [デモ](https://is.gd/bnuUxg)

----

### 配列の長さ

~~~rust
let a = [1, 2, 3];
println!("配列 a の長さ: {}", a.len());
~~~

* len() メソッドを呼ぶことで、配列の要素数を取得できます
* [デモ](https://is.gd/gCsQPu)

---

## スライス

~~~rust
let a = [0, 1, 2, 3, 4, 5];
let middle = &a[1..4];
println!("a.len() = {}", a.len());
println!("middle.len() = {}", middle.len());
~~~

* 配列中のある範囲を表す型
* ~&配列名[開始インデックス..終了インデックス]~と記述する
* [デモ](https://is.gd/TBB3nY)

---

## タプル

~~~rust
let x = (1, "hello"); // x: (i32, &str)
let mut p1 = (1, 2); // p1: (i32, i32)
let p2 = (3, 4); // p2: (i32, i32)
p1 = p2;
~~~

* 値の組みです
* 型と要素数が同じなら、代入できます 

----

### 要素へのアクセス

~~~rust
let p = (1, 2, 3);
let (x, y, z) = p;
let x = p.0;
let y = p.1;
let z = p.2;
~~~

* 分割代入をするか、インデックスを利用して要素へアクセスします

---

## 制御構造

|制御構造|文 / 式|
|------|--|
|条件分岐|if, match|
|繰り返し|for, loop, while|

* Rust の制御構文は式です（文ではありません）

---

## 条件分岐

~~~rust
let x = 5;
if x > 10 {
  println!("x > 10")
}else if x > 0{
  println!("x < x <= 10)  
}else{
  println!("x < 0")
};
~~~

* 条件節を小括弧で囲む必要はありません
* [デモ](https://is.gd/eA6B4C)

----

### if 式

~~~rust
let x = 5;
let y = if x > 0{
  1
}else{ 
  0
};
println!("y = {}", y);   
~~~

* 最後に評価した式の値が評価値となります
* ; を入れると、その後に空文があるという解釈になります
* [デモ](https://is.gd/u7ReVY)

---

## loop 文

~~~rust
let i = 0;
loop{
    println!("{} 回目の出力です", i);
    let i = i + 1;
    if i > 10{
        break;
    }     
}
~~~

* 無限ループを記述できます
* [デモ](https://is.gd/eWDb0V)


----

## while 文

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

---

## for 文

~~~ rust
for x in 0..10 {
    println!("{}", x); // x: i32
}
~~~

* `0..10` は整数の範囲を表すオブジェクトのリテラル

----

### enumerate 関数

~~~ rust
for (index, value) in (0..10).enumerate() {
    println!("index = {}, value = {}", index, value);
}
~~~

* enumerate 関数を使うと、繰り返した回数も取得できます
* タプルが帰ってきます

---

## ベクター

~~~rust
let v = vec![1, 2, 3, 4, 5];
let zeroes = vec![0; 10];
for i in (0..v.len()){
    println!("v[{}] = {}", i, v[i]);
} 
~~~

* 可変長の配列（リスト）です
* vec! マクロで作成します
* 詳細は [Vec<T>](https://doc.rust-lang.org/stable/std/vec/) を参照してください
* [デモ](https://is.gd/XDOd0T)

----

### イテレーターを利用した繰り返し

~~~rust
let v = vec![1, 2, 3, 4, 5];
for (index, value) in v.iter().enumerate() {
    println!("v[{}] = {}", i, v[i]);
} 
~~~

* iter メソッドで、ベクターをイテレーターに変換できます
* [デモ](https://is.gd/C6dys6)

----

### filter / map

~~~rust
let v = vec![1, 2, 3, 4, 5];
let result = v.iter().filter(|&n| n % 2 != 0).map(|n| n + 1);
for (index, value) in result.enumerate() {
    println!("result[{}]:{}", index, value);
}
~~~

* イテレーターは filter や map メソッドを持っています
* [デモ](https://is.gd/fzmX9R)

---

## もくもくタイム：フィボナッチ数列

* [フィボナッチ数](https://ja.wikipedia.org/wiki/%E3%83%95%E3%82%A3%E3%83%9C%E3%83%8A%E3%83%83%E3%83%81%E6%95%B0)を計算する関数 `fib` を実装してみましょう
* [テンプレートはこちら](https://is.gd/7mKTCt) 

---

## Ownership

---

## パターンマッチ

---

## cat コマンドを作ろう

~~~sh
% cat -n fileA fileB
~~~

* 指定されたファイルを標準出力へ出力するコマンドです
* 2つ以上のファイルが指定された場合は、それらを連続して出力します
* -n オプションがつけられたら、各行に行番号をつけて出力します

----

### テンプレートの作成

~~~sh
% cargo new --bin cat
~~~

* Cargo コマンドで、ファイルの雛形が作成されます

----

### ビルドと実行

~~~sh
% cd cat
% cargo run
   Compiling cat v0.1.0
    Finished debug [unoptimized + debuginfo] target(s) in 1.45 secs
     Running `target/debug/helloworld`
Hello, world!
~~~

* ```cargo run```でビルドと実行を行います
* ビルドのみ行うなら、build オプションを利用します

----

### コマンドライン引数の取得

~~~rust
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() > 1{
        println!("{}", args[1]);
    }
}
~~~

* ```[std::env::args](https://doc.rust-lang.org/std/env/fn.args.html)``` で引数をイテレーターとして取得できます
* collect メソッドで、イテレーターをベクターに変換しています
* 0 番目の要素は、実行されるコマンドの名前となっています

----

### ファイルの読み込み

~~~rust
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
~~~

* ```File::open```で作られた ```File``` オブジェクトを利用してファイルにアクセスします
* ファイルアクセスは失敗することもあるので、Result 型が返るようにメソッドを定義します

----

### 読み込んだファイルの出力

~~~rust
fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() > 1{
        println!("{}", match read_file(args[1].clone()){
            Ok(content) => content,
            Err(reason) => panic!(reason) 
        });
    }
}
~~~

* 成功 / 失敗の判断をパターンマッチを使って行なっています

----

### 発展させてみましょう！

1. 複数のファイルを続けて出力する機能を実装しましょう
2. -n オプションを実装しましょう (コマンドラインオプションの処理は [```getopts```](https://doc.rust-lang.org/getopts/getopts/index.html) を利用するとよいでしょう)

---

## grep コマンドの実装

~~~sh
% grep patterns grep.txt
     match one or more patterns.  By default, a pattern matches an input line
     Each input line that matches at least one of the patterns is written to
     grep is used for simple patterns and basic regular expressions (BREs);
     grep and egrep, but can only handle fixed patterns (i.e. it does not% grep hello
~~~      

* cat コマンドをベースに grep コマンドを実装しましょう！
* grep は指定された文字列が含まれる行を、ファイルから抜き出します

----

### 正規表現の利用（準備）

~~~
[dependencies]
regex = "0.1"
~~~

* Rust の正規表現機能は、ライブラリとして提供されています
* 上記のように regex を Cargo.toml の dependencies へと追加することで、正規表現を利用できるようになります

----

###　正規表現の利用

~~~rust
extern crate regex;
use regex::Regex;

//　中略

let re = Regex::new(r"[Hh][Ee][Ll][Ll][Oo]").unwrap();
re.is_match("HeLLo, world"); // true
~~~

* 調査したいパターンから、regex::Regex オブジェクトを作成します
* is_match メソッドで、マッチするかどうかを調査できます
* 詳しくは[こちら](https://doc.rust-lang.org/regex/regex/index.html)

---

## Have fun!

