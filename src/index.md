---
title: Rust ハンズオン
description: grep のクローンを作ることで学ぶ Rust の基礎
lang: ja-JP
layout: layouts/deck.njk
---

# Rust ハンズオン

[chikoski@](https://twitter.com/chikoski)

## このハンズオンについて

* Rust とは？から始まって、基本的な文法をカバーします
* 自分なりの grep コマンドを実装します
* 自分のペースで、できるところまでやりましょう

### 今回の成果物：grep

* テキストファイルから、パターンに一致する行を抜き出すコマンド
* 使い方：`grep オプション パターン ファイル名`
* 次の例では、`Cargo.toml` の中から、`2018` という文字列が含まれる行を抜き出しています：

~~~shell-session
% grep 2018 Cargo.toml
edition = "2018"
~~~

### 資料の使い方

* 資料はスライド形式になっています
* 横方向に進んでいけば、自分なりの grep を実装できます
* 縦方向にも進める場合もあります

#### 縦方向のスライドは補足です

* 縦方向のスライドには、補足や詳細な情報がのっています

### 本日の内容

0. Rust についての簡単な紹介
1. FizzBuzz を作ろう
2. テキストファイルを表示するプログラムを作ろう
3. 2 を改造して、grep コマンドを作ろう

### Rust とは

* システムプログラミング用の言語としてスタートしました
* 信頼性が高く、性能の良いプログラムを書けるように設計されています
* 組み込みからWeb まで、さまざまな場面で利用されています

#### Rust の採用例

* [Friends of Rust](https://prev.rust-lang.org/en-US/friends.html) / [Production](https://www.rust-lang.org/production)
* [Awesome Rust](https://github.com/awesome-rust-com/awesome-rust)
* 日本の例：LINE、Voyage group、CADDi、Cookpad、Dwango、Forcia、etc

#### ドキュメント

* [The book](https://doc.rust-lang.org/book/) / [日本語版](https://doc.rust-jp.rs/book-ja/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) / [日本語版](https://doc.rust-jp.rs/rust-by-example-ja/)
* [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)

## FizzBuzz を作ろう

* 1 から 100 までの数値を出力します
* ただし、次の場合は数字の代わりに指定された文字列を出力します
  * 数字が 3 の倍数の場合、Fizz を出力します
  * 数字が 5 の倍数の場合、Buzz を出力します
  * 数字が 3 の倍数で、5 の倍数でもある場合には、FizzBuzz を出力します


### プロジェクトの作成

* Rust のプロジェクトの操作には、cargo コマンドを使います
* `cargo new` を実行すると、プロジェクトが作成されます

~~~shell-session
% cargo new fizzbuzz
     Created binary (application) `fizzbuzz` package
~~~

#### プロジェクトのフォルダ構成

~~~shell-session
fizzbuzz
├── Cargo.toml
└── src
   └── main.rs
~~~

### プロジェクトのビルドと実行

* `cargo build` でビルドします
* `cargo run` で、ビルドしたプログラムを実行します
  * ビルドされていない場合は、ビルドも行います
  * 標準ではデバッグビルドを実行します

~~~shell-session
% cd fizzbuzz
% cargo run
   Compiling fizzbuzz v0.1.0 (/Users/chikoski/fizzbuzz)
    Finished dev [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/fizzbuzz`
Hello, world!
~~~

### main.rs

* main 関数が定義されています
* この関数（main）がエントリーポイントです
* [println!](https://doc.rust-lang.org/std/macro.println.html) は文字列を出力し、最後に改行するマクロです

~~~rust
fn main() {
    println!("Hello, world!");
}
~~~

### 繰り返し：while 文

* [`let` は変数に値を束縛するキーワード](https://doc.rust-lang.org/std/keyword.let.html)です
* デフォルトでは、変数の値を変更できません
* 値を変更する変数には、[`mut` キーワード](https://doc.rust-lang.org/std/keyword.mut.html)をつけます

~~~rust
fn main() {
  let mut counter = 0;
  while counter < 10{
    println!("Hello, world!");
    counter += 1;
  }
}
~~~

### イテレーターを使った書き換え

* `0..10` は 0 以上 10 未満の[範囲を表すオブジェクト](https://doc.rust-lang.org/std/ops/struct.Range.html)を定義します
* このオブジェクトは、[イテレーター](https://ja.wikipedia.org/wiki/%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF)としての性質を持っています
* そのため [for 文](https://ja.wikipedia.org/wiki/%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF)で範囲に含まれる整数を列挙できます
* 下記では、列挙された値を使わないので、`_` を利用しています

~~~rust
fn main() {
  for _ in 0..10{
    println!("Hello, world!");
  }
}
~~~

### 条件分岐：if 式

* Rust での if は式です。つまり評価値を持ちます
* 実行したブロックで最後に評価した式の値が、if 式の評価値となります
* ブロックの最後の式に `;` がついていないことがポイントです

~~~rust
fn main() {
  for n in 0..10{
    let output = if n % 15 == 0{
      "FizzBuzz"
    }else{
      format!("{}", n)
    };
    println!("{}", output);
  }
}
~~~

#### `format`

* [`format`](https://doc.rust-lang.org/std/fmt/fn.format.html) は第 1 引数に指定した書式に、第 2 引数以降の値を埋め込んだ文字列を返すマクロです
* 次の例では、`Hello, {}` の `{}` 部分に、`name` の値が埋め込まれます

~~~rust
fn main(){
  let name = "World";
  let output = format!("Hello, {}!", name);
  println!("{}", output);
}
~~~

### コンパイル時の型チェック

* コンパイル時には型チェックが行われます
* 次の例では、if 節の評価値が `str` 型なのに対し、else の評価値が `String` 型であることが原因でコンパイルエラーとなっています
* `"FizzBuzz"` に対して、`to_string` メソッドを呼ぶように変更することで修正できます

~~~shell-session
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:6:9
  |
3 |         let output = if n % 15 == 0{
  |  ____________________-
4 | |         "FizzBuzz"
  | |         ---------- expected because of this
5 | |       }else{
6 | |         format!("{}", n)
  | |         ^^^^^^^^^^^^^^^^ expected `&str`, found struct `String`
7 | |       };
  | |_______- `if` and `else` have incompatible types
  |
~~~  

#### String と str

* `String` は文字列を表すオブジェクトです
* `str` は文字列のスライスを表す値です
* `to_string` メソッドで、`String` オブジェクトへ変換できます

~~~rust
let name = "World";
let message = format!("Hello, {}!", name);
println!("{}", message);
   
let slice_of_message = &message[0..5];
println!("{}", slice_of_message);

let another_string = slice_of_message.to_string();
println!("{}", another_string);
~~~

#### コンパイルエラーの修正

* if 節の評価値の型が String となるように修正します
* `to_string` メソッドを呼ぶことで、String 型の `"FizzBuzz"` を作成できます

~~~rust
fn main() {
  for n in 0..10{
    let output = if n % 15 == 0{
      "FizzBuzz".to_string()
    }else{
      format!("{}", n)
    };
    println!("{}", output);
  }
}
~~~

#### より詳しくコンパイルエラーについて知りたい場合は

* rustc に --explain オプションをつけて実行すると、より詳しい解説を読めます
* 次の例では、E0308 のエラーについて、解説を読みます
* 同じ解説を [Web でも読めます](https://doc.rust-lang.org/error-index.html)

~~~shell-session
% rustc --explain E0308
Expected type did not match the received type.

Erroneous code examples:
~~~

### FizzBuzz: 手続き的なバージョン

* 以下は、手続き的に書いた FizzBuzz の例です
* これを少しずつ改変し、Rust の柔軟性をみていきます

~~~rust
fn main() {
  for n in 1..20{
    let output = if n % 15 == 0{
      "FizzBuzz".to_string()
    }else if n % 5 == 0{
      "Buzz".to_string()
    }else if n % 3 == 0{
      "Fizz".to_string()
    }else{
      format!("{}", n)
    };
    println!("{}", output);
  }
}
~~~

### 関数への切り出し

* [`fn` キーワード](https://doc.rust-lang.org/std/keyword.fn.html) で関数を定義できます
* `()` 内に引数のリストを、`->` の後に返り値の型を書きます
* 下記は、FizzBuzz の数値から文字列への変換を、関数に切り出した例です
* [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) を String への変換として実装しています

~~~rust
fn fizzbuzz(n: u32) -> String{
  if n % 15 == 0{
    "FizzBuzz".to_string()
  }else if n % 5 == 0{
    "Buzz".to_string()
  }else if n % 3 == 0{
    "Fizz".to_string()
  }else{
    format!("{}", n)
  }
}
~~~

#### 関数呼び出し

* 実引き数をを与えて関数を呼びます
* 型を明記していないのは、コンパイラーが型推論を行うためです

~~~rust
fn main() {
  for n in 1..20{
    let output = fizzbuzz(n);
    println!("{}", output);
  }
}
~~~

#### 単体テスト

* `#[test]` とアノテーションされた関数は、テスト用の関数として処理されます
* 次の例では、`fizzbuzz` の振る舞いをテストしています
* `cargo test` でテストを実行できます

~~~rust
#[test]
fn test_fizzbuzz_returns_fizzbuzz() {
    let expected = "FizzBuzz".to_string();
    let actual = fizzbuzz(15);
    assert_eq!(expected, actual);
}
~~~

### 変換部分を関数に切り出した結果

~~~rust
fn fizzbuzz(n: u32) -> String{
  if n % 15 == 0{
    "FizzBuzz".to_string()
  }else if n % 5 == 0{
    "Buzz".to_string()
  }else if n % 3 == 0{
    "Fizz".to_string()
  }else{
    format!("{}", n)
  }
}

fn main() {
  for n in 1..20{
    let output = fizzbuzz(n);
    println!("{}", output);
  }
}
~~~

### FizzBuzz：関数プログラミング的なアプローチ

* FizzBuzz はデータ変換を行う関数として捉えることもできます
* 例：数値の範囲 -> 文字列の配列
* 一つ一つの数値を、文字列に変換する関数は fizzbuzz として用意されています
* これを使って、関数プログラミング的なアプローチで FizzBuzz を書き直します

#### FizzBuzz: map メソッド

* コレクション中の要素一つ一つに関数を適用して、別のコレクションを作る map と呼ばれる操作は、関数プログラミングで良く利用されます
* Rust のイテレーターにも `map` メソッドは用意されています
* このメソッドは、各要素に、引数に与えた関数を適用した結果を持つイテレーターを返します

~~~rust
fn main() {
  for output in (1..20).map(fizzbuzz){
    println!("{}", output);
 }
}
~~~

#### クロージャー：関数の一種

* クロージャーは関数の一種で、定義された時にアクセス可能な変数であれば関数本体内で利用できる、という点が特徴です
* 無名関数やラムダ、といった名前でクロージャーを提供している言語もあります
* Rust でもクロージャーは利用できます。下記の例では、 `fold` メソッドの第 2 引数でクロージャーを定義しています
* 仮引数リストは `|` と `|` の間に記述します

~~~rust
fn main() {
  let output = (1..20).map(fizzbuzz)
      .fold("".to_string(), |accum, line|{
    format!("{}\n{}", accum, line)
  });
  println!("{}", output);
}
~~~

### 関数プログラミング的な FizzBuzz

~~~rust
fn fizzbuzz(n: u32) -> String{
  if n % 15 == 0{
    "FizzBuzz".to_string()
  }else if n % 5 == 0{
    "Buzz".to_string()
  }else if n % 3 == 0{
    "Fizz".to_string()
  }else{
    format!("{}", n)
  }
}

fn main() {
  let output = (1..20).map(fizzbuzz).fold("".to_string(), |accum, line|{
    format!("{}\n{}", accum, line)
  });
  println!("{}", output);
}
~~~

### FizzBuzz のまとめ

* 基本的な文法を確認しました
* 手続き的にも、関数型的にも書けます
* コンパイラーを良いレビュワーとして付き合っていけると良いと思います

## テキストファイルを出力するプログラムを作ろう

1. 新しいプロジェクトを作ります
2. 決まったファイルの中身を出力するプログラムを作ります
3. コマンドライン引数で、読むファイルを指定できるように変更します

### プロジェクトの作成

* `mygrep` という名前のプロジェクトを作ります
* 次の例では、ホームディレクトリにプロジェクトを作成しています

~~~shell-session
% cd
% cargo new mygrep
~~~

### 自分自身を出力するプログラム

* 手始めに自分自身を出力するプログラムを作成します
* [`std::fs:read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) は、引数で指定したパスから文字列として内容を読み込みます

~~~rust
fn main() {
  let path = "./src/main.rs";
  
  match std::fs::read_to_string(path) {
    Ok(content) => print!("{}", content),
    Err(reason) => println!("{}", reason)
  }
}
~~~

### ファイル読み込みには失敗がつきもの

* Rust では、ある操作の成否を [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) を使って表現します
* `Result` は成功を表す `Ok` と エラーを表す `Err` から成る enum です
* `is_ok` メソッドで、`Ok` か `Err` かを判別できます

~~~rust
fn main() {
  let result: Result<u32, String> = Ok(1);
  let message = if result.is_ok(){
    "Success"
  }else{
    "Fail"
  };
  println!("{}", message);
}
~~~

#### 成果物とエラーの理由

* `Ok` は成果物を値として持てます。* 同様に `Err` も、エラーの理由を値として持てます
* 成果物を表すデータ型や、エラーの理由を表すデータ型は、プログラムによって異なります
* そのため `Result` を扱う際には、成果物のデータ型とエラーの理由を表すデータ型もあわせて指定します
* 次の例では、成果物の型に `u32` を指定し、エラーの理由は `String` で与えられるとしています

~~~
let result: Result<u32, String> = Ok(1);
~~~

#### 成果物の取り出し方（unwrap を使う場合）

* `unwrap` メソッドを使って、成果物（もしくはエラーの理由）を取り出せます
* 次の例では、成功した場合に `unwrap` メソッドを使って成果物を取り出しています

~~~rust
fn main() {
  let result: Result<u32, String> = Ok(1);
  let message = if result.is_ok(){
    format!("Result = {}", result.unwrap())
  }else{
    "Fail".to_string()
  };
  println!("{}", message);
}
~~~

#### 成果物の取り出し方（パターンマッチを使う場合）

* 条件分岐と `unwrap` との組み合わは、パターンマッチを使って簡略化できます
* パターンマッチは [match 式](https://doc.rust-lang.org/std/keyword.match.html) として記述できます
* パターンの一部分を、変数に束縛することで、`Ok` / `Err` から値を取り出せます

~~~rust
fn main() {
  let result: Result<u32, String> = Ok(1);
  let message = match result {
    Ok(value) => format!("Result = {}", value),
    Err(_) => "Fail".to_string()
  };
  println!("{}", message);
}
~~~

#### Result のエイリアス：std::io::Result

* エラーの理由を表す型が決まっている、といった理由で Result のエイリアスが作られることは良くあります
* 代表例は、[std::io::Result](https://doc.rust-lang.org/std/io/type.Result.html) です
* 次のように、エラーを std::io::Error で表すと定めています

~~~rust
type Result<T> = Result<T, std::io::Error>;
~~~

### 内容を出力する部分を関数へ切り出し

* 出力するファイルを指定できるようにするための準備として、内容を出力する部分を関数に切り出します
* ファイルのパスを `String` で受け取り、`()` を返す関数として定義しました
* これにあわせて `path` の型が `String` になっている点に注意してください

~~~rust
fn run(path: String){
  match std::fs::read_to_string(path) {
    Ok(content) => print!("{}", content),
    Err(reason) => println!("{}", reason)
  }  
}

fn main() {
  let path = "./src/main.rs".to_string();
  run(path);
}
~~~

### 出力するファイルの指定方法

* 出力するファイルを、コマンドライン引数として指定することとします
* 実行のイメージは次のようになります
* この例では `src/main.rs` を出力します

~~~shell-session
% cargo run src/main.rs
~~~

### コマンドライン引数の取得

* [`std::env::args()`](https://doc.rust-lang.org/std/env/fn.args.html) は、コマンドライン引数を String として保持するイテレーターを返します
* [`nth`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth) メソッドで、n 番目の要素を取得できます
* `nth` メソッドは `Option` という値を返します

~~~rust
fn main() {
  let arguments = std::env::args();
  match arguments.nth(1){
    Some(path) => run(path),
    None => println!("No path is specified"),
  }  
}
~~~

### Option: null かもしれない値の表現

* [Option](https://doc.rust-lang.org/std/option/index.html) は存在するかもしれないし、しないかもしれないといった値を表現します
* 下記の例では、1 番目のコマンドライン引数を取得しています
* この値が存在するかどうかは、ユーザーの入力に依存します

~~~rust
fn main() {
  let arguments = std::env::args();
  match arguments.nth(1){
    Some(path) => run(path),
    None => println!("No path is specified"),
  }  
}
~~~

#### Option の値：Some と None

* Option は Result と良く似た性質をしています
* 公式ドキュメントでは、Result はリッチな Option として紹介されています
* [値が存在する場合、Option の値は `Some`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some) となります。[値が存在しない場合の値は `None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) です

~~~rust
fn main() {
  let arguments = std::env::args();
  match arguments.nth(1){
    Some(path) => run(path),
    None => println!("No path is specified"),
  }  
}
~~~

#### Option からの値取得

* Some は、実際の値を内部に保持しています
* Option は Result と同様に、`unwrap` メソッドがあります。これを利用して実際の値を取得できます
* また下記のようにパターンマッチを利用しても、保持されている値を取得できます

~~~rust
fn main() {
  let arguments = std::env::args();
  match arguments.nth(1){
    Some(path) => run(path),
    None => println!("No path is specified"),
  }  
}
~~~

### ここまででできたプログラム

~~~rust
fn run(path: String){
  match std::fs::read_to_string(path) {
    Ok(content) => print!("{}", content),
    Err(reason) => println!("{}", reason)
  }  
}

fn main() {
  let arguments = std::env::args();
  match arguments.nth(1){
    Some(path) => run(path),
    None => println!("No path is specified"),
  }  
}
~~~

## grep への拡張

* ここまでで、指定したファイルの中身を文字列として出力するプログラムができました。これを拡張して grep コマンドを実装します
* grep には 2 つのコマンドライン引数があります。1 つ目がパターン、2 つめがファイルパスです
* 次のれいでは、version がパターンで、Cargo.toml がファイルパスとなります

~~~shell-session
% grep version Cargo.toml
version = "0.1.0"
~~~

### run を変更して grep を実装します

~~~rust
fn grep(content: String, pattern: String){
  for line in content.lines(){
    if line.contains(pattern.as_str()){
      println!("{}", line);
    }
  }
}

fn run(path: String, pattern: String){
  match std::fs::read_to_string(path) {
    Ok(content) => grep(content, pattern),
    Err(reason) => println!("{}", reason)
  }  
}
~~~

### パターンとファイルパスの取得

* `std::env::args` の返り値はイテレーターとしての性質を持っています
* `nth` メソッドを呼ぶたびに、内部の状態が変わります
* 状態変化による面倒を避けるため、都度 `std::env::args` を呼んでいます

~~~rust
fn main() {
  let pattern = std::env::args().nth(1);
  let path = std::env::args().nth(2);
  if pattern.is_some() && path.is_some(){
    run(path, pattern);
  }
}
~~~

### ユーザー定義型

* 今後の発展のために、`path` と `pattern` をまとめたデータ構造を作ります
* データ構造は [`struct` キーワード](https://doc.rust-lang.org/std/keyword.struct.html)を使って定義できます

~~~rust
struct MyGrep{
  path: String,
  pattern: String,
}
~~~

### impl：データ構造に振る舞いを与えるキーワード

* [`impl` キーワード](https://doc.rust-lang.org/std/keyword.impl.html)を使うと、データ構造に振る舞いを定義できます
* 例えば、データ構造を作成する関数 new は次のように定義できます

~~~rust
struct MyGrep{
  path: String,
  pattern: String,
}

impl MyGrep{
  fn new(path: String, pattern: String) -> MyGrep{
    MyGrep{
      path, 
      pattern,
    }
  }
}
~~~

### MyGrep 型を使った書き換え

* 定義した MyGrep 型を使うようにコードを書き換えます
* 書き換えるのは、main と run の 2 関数です
* 変数名とフィールド名を `.` でつなぐとフィールドの値を参照できます

~~~rust
fn run(mygrep: MyGrep){
  match std::fs::read_to_string(mygrep.path) {
    Ok(content) => grep(content, mygrep.pattern),
    Err(reason) => println!("{}", reason)
  }  
}

fn main(){
  let pattern = std::env::args().nth(1);
  let path = std::env::args().nth(2);

  if pattern.is_some() && path.is_some() {
    run(MyGrep::new(path.unwrap(), pattern.unwrap()))
  }
}
~~~

### ここまでの状態

~~~rust
struct MyGrep{
  path: String,
  pattern: String,
}

impl MyGrep{
  fn new(path: String, pattern: String) -> MyGrep{
    MyGrep{
      path, 
      pattern,
    }
  }
}

fn grep(content: String, pattern: String){
  for line in content.lines(){
    if line.contains(pattern.as_str()){
      println!("{}", line);
    }
  }
}

fn run(mygrep: MyGrep){
  match std::fs::read_to_string(mygrep.path) {
    Ok(content) => grep(content, mygrep.pattern),
    Err(reason) => println!("{}", reason)
  }  
}

fn main(){
  let pattern = std::env::args().nth(1);
  let path = std::env::args().nth(2);

  if pattern.is_some() && path.is_some() {
    run(MyGrep::new(path.unwrap(), pattern.unwrap()))
  }
}
~~~

### コマンドラインオプションに対応しよう

* grep は `-n` オプションをつけると、行番号をつけて結果を出力します。この機能を実装します
* オプションの解析には、ライブラリを利用することします
* ライブラリのことを、Rust では crate（クレート）と呼びます

### クレートの追加

* 今回は [structopt](https://github.com/TeXitoi/structopt) という crate を使います
* Rust のライブラリは、[crates.io](https://crates.io/) というレポジトリにまとめられています
* Cargo.toml の dependencies に、使用する crate を追記することで利用できるようになります

~~~toml
[dependencies]
structopt = "0.3.21"
~~~

### コマンドラインオプションの解析

* structopt を使うと、コマンドラインオプションの解析を宣言的に記述できます
* 定義したデータ構造の各フィールドにアトリビュートを追加することで、コマンドラインオプションとの対応関係を記述します
* 以下は、MyGrep 型にアトリビュートを追加した例です

~~~rust
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name="mygrep")]
struct MyGrep{
  #[structopt(name = "PATTERN")]
  pattern: String,
  #[structopt(name = "FILE")]
  path: String,
}
~~~

### コマンドラインオプションの解析（つづき）

* `MyGrep::from_args` は structopt によって追加されました
* この関数が、コマンドラインオプションの解析と、MyGrep オブジェクトを作成します

~~~rust
fn main(){
  let mygrep = MyGrep::from_args();
  run(mygrep);
}
~~~

### ここまでの状態（Cargo.toml）

~~~toml
[package]
name = "mygrep"
version = "0.1.0"
authors = ["自分のなまえ"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
structopt = "0.3.21"
~~~

### ここまでの状態（main.rs）

* `MyGrep::new` は使わなくなったので、削除しました

~~~rust
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name="mygrep")]
struct MyGrep{
  #[structopt(name = "PATTERN")]
  pattern: String,
  #[structopt(name = "FILE")]
  path: String,
}

fn grep(content: String, pattern: String){
  for line in content.lines(){
    if line.contains(pattern.as_str()){
      println!("{}", line);
    }
  }
}

fn run(mygrep: MyGrep){
  match std::fs::read_to_string(mygrep.path) {
    Ok(content) => grep(content, mygrep.pattern),
    Err(reason) => println!("{}", reason)
  }  
}

fn main(){
  let mygrep = MyGrep::from_args();
  run(mygrep);
}
~~~

#### Trait：データ構造の満たす性質の定義

* Rust は型を、そのデータ構造が満たす性質で区別します
* データ構造の満たす性質は、インタフェースの集まりとして定義されます
* インタフェースの集まりのことを、Rust では trait と呼びます
* 次の例では、`i32` と `Vec` は、ともに `Zero` という trait を実装しているため、同じ型として扱えます

~~~rust
trait Zero {
    fn is_zero(&self) -> bool;
}

impl Zero for i32 {
    fn is_zero(&self) -> bool {　*self == 0　}
}

impl<T> Zero for Vec<T>{
    fn is_zero(&self) -> bool{ self.len() == 0 }
}
~~~

#### Derive アトリビュート：コードの自動的な追加

* [Derive アトリビュート](https://doc.rust-lang.org/reference/attributes/derive.html) は、指定した trait の持つインタフェースを実装します
* 次の例では、`Foo` に [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) と [Clone](https://doc.rust-lang.org/std/clone/index.html) を実装しています
* [実装方法はマクロによって定められ](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes)ています
* マクロが用意されていない trait を Derive アトリビュートを使って実装できません

~~~rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
~~~

#### use：名前空間へのシンボルの追加

* [`use` は、別のファイルで定義されているシンボルを、そのファイルの名前空間に追加](https://doc.rust-lang.org/std/keyword.use.html)でき、シンボルを省略して記述できるようになります
* 次の例では、`structopt::StructOpt` を、そのファイルの名前空間に追加したため、[`StructOpt` trait](https://docs.rs/structopt/0.3.21/structopt/trait.StructOpt.html) を `StructOpt` で参照できています
* 仮に名前空間に追加しなかった場合、`StructOpt` を解決できず、コンパイルエラーとなります

~~~rust
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name="mygrep")]
struct MyGrep{
  #[structopt(name = "PATTERN")]
  pattern: String,
  #[structopt(name = "FILE")]
  path: String,
}
~~~

### `-n` オプションの追加

* `line_number` という bool 型のフィールドを追加します
* 追加したフィールドにも、structopt アトリビュートを追加します
* `short` はショートオプション、`long` はロングオプションを受け付けるための記述です

~~~rust
struct MyGrep{
  #[structopt(name = "PATTERN")]
  pattern: String,
  #[structopt(name = "FILE")]
  path: String,
  #[structopt(short = "-n", long)]
  line_number: bool,
}
~~~

### オプションが追加されたことの確認

* 次のように `--help` オプションをつけて、cargo コマンドを実行し、`-n` オプションが存在することを確認します
* `--help` や `--version` は、structopt を Derive した際に追加されています

~~~shell-session
% cargo run -- --help
（中略）
mygrep 0.1.0

USAGE:
    mygrep [FLAGS] <PATTERN> <FILE>

FLAGS:
    -h, --help           Prints help information
    -n, --line-number
    -V, --version        Prints version information

ARGS:
    <PATTERN>
    <FILE>
~~~

### `-n` を実装します

* grep の第 1 引数を、MyGrep オブジェクトに変更します

~~~rust
fn grep(mygrep: MyGrep, content: String){
  let mut line_number = 1;
  for line in content.lines(){
    if line.contains(mygrep.pattern.as_str()){
      if mygrep.line_number {
        println!("{}: {}", line_number, line);
      }else{
        println!("{}", line);
      }
    }
    line_number += 1;
  }
}
~~~

### grep のシグネチャ変更にあわせて、run も変更します

~~~rust
fn run(mygrep: MyGrep){
  match std::fs::read_to_string(mygrep.path) {
    Ok(content) => grep(mygrep, content),
    Err(reason) => println!("{}", reason)
  }  
}
~~~

### 所有権の移動に伴うコンパイルエラー

* `mygrep.path` の値は `read_to_string` の引数に[所有権](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)が移動しています
* 所有権が移動したフィールドをもつオブジェクトを利用しているので、コンパイルエラーがおきます
* 所有権は代入や、関数呼び出しによって移動します

~~~shell-session
error[E0382]: use of partially moved value: `mygrep`
  --> src/main.rs:68:25
   |
67 |   match std::fs::read_to_string(mygrep.path) {
   |                                 ----------- value partially moved here
68 |     Ok(content) => grep(mygrep, content),
   |                         ^^^^^^ value used here after partial move
   |
   = note: partial move occurs because `mygrep.path` has type `String`, which does not implement the `Copy` trait

error: aborting due to previous error

For more information about this error
~~~

### 参照と借用

* 所有権を渡す代わりに、一時的に値を貸し出す、ということもできます
* 貸し出す場合は、値は[参照](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)という形で渡します
* 変数名の前に `&` をつけることで、値への参照を取得します

~~~rust
fn run(mygrep: MyGrep){
  match std::fs::read_to_string(&mygrep.path) {
    Ok(content) => grep(mygrep, content),
    Err(reason) => println!("{}", reason)
  }  
}
~~~

### ここまでの状態

~~~rust
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name="mygrep")]
struct MyGrep{
  #[structopt(name = "PATTERN")]
  pattern: String,
  #[structopt(name = "FILE")]
  path: String,
  #[structopt(short = "-n", long)]
  line_number: bool,
}

fn grep(mygrep: MyGrep, content: String){
  let mut line_number = 1;
  for line in content.lines(){
    if line.contains(mygrep.pattern.as_str()){
      if mygrep.line_number {
        println!("{}: {}", line_number, line);
      }else{
        println!("{}", line);
      }
    }
    line_number += 1;
  }
}

fn run(mygrep: MyGrep){
  match std::fs::read_to_string(&mygrep.path) {
    Ok(content) => grep(mygrep, content),
    Err(reason) => println!("{}", reason)
  }  
}

fn main(){
  let mygrep = MyGrep::from_args();
  run(mygrep);
}
~~~

### `-r` オプションを実装しよう

* `-r` オプションは、指定されたパターンを正規表現として解釈するオプションです
* 正規表現は [regex](https://crates.io/crates/regex) クレートで実現されています
* まず `Cargo.toml` を編集して regex クレートを読み込んだ上で、`main.rs` に機能を実装します

### `run` を `MyGrep` のメソッドにしよう

* `run` はトップレベルに実装された関数でした
* これを `MyGrep` のメソッドとなるように変更します
* 変更後の、`main` 関数は次のようになります

~~~rust
fn main(){
  let mygrep = MyGrep::from_args();
  mygrep.run();
}
~~~

#### `self` を引数にとる関数

* [`self`](https://doc.rust-lang.org/std/keyword.self.html) を引数にとる関数をデータ構造に実装することで、そのデータ構造にメソッドを追加できます
* メソッド呼び出し時、`self` はメソッドのレシーバーに束縛されます
* 次の例では、`Peano` という構造体に `succ` というメソッドを追加しています

~~~rust
struct Peano{
  value: u32
}
impl Peano{
  fn zero() -> Peano{ 
    Peano{value: 0}
  }
  fn succ(&self) -> Peano{
    Peano{value: self.value + 1}
  }
}
fn main(){
  let zero = Peano::zero();
  let one = zero.succ();
  println!("zero = {}, one = {}", zero.value, one.value);
}
~~~

#### `self` か `&self` か

* メソッドの第 1 引数は、`self` もしくは `&self` となります
* 前者ではメソッド呼び出し時に、自身の所有権が仮引数に移動します。後者では、所有権が移動しません

~~~rust
struct Peano{
  value: u32
}
impl Peano{
  fn zero() -> Peano{ 
    Peano{value: 0}
  }
  fn succ(self) -> Peano{
    Peano{value: self.value + 1}
  }
}
fn main(){
  let zero = Peano::zero();
  let one = zero.succ();
}
~~~

#### メソッド呼び出しによる所有権の移動

* `succ` を呼び出した時点で `zero` に束縛されていた値の所有権が移動しています
* 一方 `println!` では所有権の移動した値を参照しています
* そのため以下の例は、コンパイルエラーとなります

~~~rust
struct Peano{
  value: u32
}
impl Peano{
  fn zero() -> Peano{ 
    Peano{value: 0}
  }
  fn succ(self) -> Peano{
    Peano{value: self.value + 1}
  }
}
fn main(){
  let zero = Peano::zero();
  let one = zero.succ();
  println!("zero = {}, one = {}", zero.value, one.value);
}
~~~  
