# Rust 初心者向けハンズオン

Mozilla Japan 

清水智公 (nshimizu@mozilla-japan.org)

---

## 今日のゴール

* Rust っていい言語だなーと思ってもらうこと：）
* トピック
   * 基本的な構文
   * owenership と borrowing
   * ユーザー定義型（struct, enum, trait）
   * 並列処理（メッセージパッシングと共有メモリー） 

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
    * 静的型づけ、型推論
    * 安全なポインター操作（ownership、move semantics、borrowing）
    * 抽象データ型
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

* [コンパイルエラー](ttps://is.gd/kieuKz)になります
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

