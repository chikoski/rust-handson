# Rust 初心者向けハンズオン

Mozilla Japan 

清水智公 (nshimizu@mozilla-japan.org)

---

## 今日のゴール

* Rust っていい言語だなーと思ってもらうこと
* トピック
   * 基本的な構文
   * owenership と borrowing
   * 並列処理（メッセージパッシングと共有メモリー）
* 簡単な [cat コマンド](http://itpro.nikkeibp.co.jp/article/COLUMN/20060227/230725/)と、同じく簡単な [grep コマンド](http://itpro.nikkeibp.co.jp/article/COLUMN/20060227/230786/)を実装します     

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
* Windows の場合：[インストーラー](https://www.rust-lang.org/en-US/downloads.html)で、インストールします

----

### インストールされるツール

|ツール名|説明|
|------|----|
|rustc|Rust のコンパイラー|
|cargo|パッケージ管理ツール|

* ビルドや実行は cargo を使って行います

---

## Hello World

<pre><code data-source="samples/helloworld/src/main.rs" data-trim class="hljs rust"></code></pre>

* [こちらで実行できます](https://is.gd/w9hWRA)
* 一から書きたい場合は [こちら](https://is.gd/LR2nNa)

----

### main 関数

<pre><code data-source="samples/helloworld/src/main.rs" data-trim class="hljs rust"></code></pre>

* Rust のプログラムは、main 関数から実行が始まります
* fn は関数を定義する文です

----

### println!

<pre><code data-source="samples/helloworld/src/main.rs" data-trim class="hljs rust"></code></pre>

* リテラルを標準出力へ出力するマクロ
* 変数の埋め込みにも対応

---

## 変数

<pre><code data-source="samples/bindings/src/main.rs" data-trim class="hljs rust"></code></pre>

* 変数は宣言してから利用します（束縛 / 参照）
* let 文で変数名を宣言できます
* [デモ](https://is.gd/rvBz9t)

----

### 変数の束縛

<pre><code data-source="samples/bindings/src/main.rs" data-trim class="hljs rust"></code></pre>

* 束縛：値と名前を結びつけること
* 便宜上、代入と呼ぶこともあります

----

### 再代入は原則できません

<pre><code data-source="samples/re-assignment/src/main.rs" data-trim class="hljs rust"></code></pre>

* [コンパイルエラー](https://is.gd/kieuKz)になります
* let 文で宣言された変数は変更できません

----

### 再代入に対するコンパイルエラー

~~~
error[E0384]: re-assignment of immutable variable `name`
 --> reassignment.rs:4:5
  |
2 |     let name = "Chiko";
  |         ---- first assignment to `name`
3 |     println!("Hello, {}", name);
4 |     name = "Shimizu";
  |     ^^^^^^^^^^^^^^^^ re-assignment of immutable variable
~~~

----

### 再び束縛する

<pre><code data-source="samples/re-binding/src/main.rs" data-trim class="hljs rust"></code></pre>

* これは[コンパイルエラーが起きません](https://is.gd/m79jCx)
* 束縛を作り直しています（代入ではないことに注意）

----

### 変更可能な変数

<pre><code data-source="samples/mutable-variable/src/main.rs" data-trim class="hljs rust"></code></pre>

* mut 修飾子をつけると、変更可能な変数が宣言されます
* [デモページ](https://is.gd/hx0FuE)

---

### 関数宣言

<pre><code data-source="samples/function/src/main.rs" data-trim class="hljs rust"></code></pre>

* 引数と返り値の型を明示します
* [動作例](https://is.gd/JPFez0)

----

### 変数に対する型アノテーション

<pre><code data-source="snippets/type-annotation.rs" data-trim class="hljs rust"></code></pre>

* ```変数:型``` と書くことで変数の型を明示できます
* 型が推論できる場合は、型アノテーションは省略できます

----

### 返り値

<pre><code data-source="snippets/signature.rs" data-trim class="hljs rust"></code></pre>

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
|'static str, str, String|文字列|

---

## 配列

<pre><code data-source="snippets/array/literal.rs" data-trim class="hljs rust"></code></pre>

* 固定長で、要素は全て同じデータ型です
* ```[型名; 要素数]``` と型アノテーションします
* 要素を変更するためには mut 修飾子が必要です

----

### 要素の参照

<pre><code data-source="snippets/array/items.rs" data-trim class="hljs rust"></code></pre>

* 添え字で、個々の要素へアクセスします
* 先頭の添え字は 0 です
* [デモ](https://is.gd/bnuUxg)

----

### 配列の長さ

<pre><code data-source="snippets/array/length.rs" data-trim class="hljs rust"></code></pre>

* len() メソッドを呼ぶことで、配列の要素数を取得できます
* [デモ](https://is.gd/gCsQPu)

---

## スライス

<pre><code data-source="snippets/array/slice.rs" data-trim class="hljs rust"></code></pre>

* 配列中のある範囲を表す型
* ```&配列名[開始インデックス..終了インデックス]```と記述する
* [デモ](https://is.gd/TBB3nY)

---

## タプル

<pre><code data-source="snippets/tuple/basics.rs" data-trim class="hljs rust"></code></pre>

* 値の組みです
* 型と要素数が同じなら、代入できます 

----

### 要素へのアクセス

<pre><code data-source="snippets/tuple/items.rs" data-trim class="hljs rust"></code></pre>

* 分割代入をするか、インデックスを利用して要素へアクセスします

---

## 制御構造

|制御構造|文 / 式|
|------|--|
|条件分岐|if, match|
|繰り返し|for, loop, while|

---

## 条件分岐

<pre><code data-source="snippets/if/basics.rs" data-trim class="hljs rust"></code></pre>

* 条件節を小括弧で囲む必要はありません
* [デモ](https://is.gd/eA6B4C)

----

### if 式

<pre><code data-source="snippets/if/expression.rs" data-trim class="hljs rust"></code></pre>

* 最後に評価した式の値が評価値となります
* ; を入れると、その後に空文があるという解釈になります
* [デモ](https://is.gd/u7ReVY)

---

## loop 文

<pre><code data-source="snippets/loop.rs" data-trim class="hljs rust"></code></pre>

* 無限ループを記述できます
* [デモ](https://is.gd/s7SLib)


----

## while 文

<pre><code data-source="snippets/while.rs" data-trim class="hljs rust"></code></pre>

---

## for 文

<pre><code data-source="snippets/iterator/for.rs" data-trim class="hljs rust"></code></pre>

* `0..10` は整数の範囲を表すオブジェクトのリテラル
* イテレーターを捜査する、という目的が主

----

### enumerate 関数

<pre><code data-source="snippets/enumerator/for.rs" data-trim class="hljs rust"></code></pre>

* enumerate 関数を使うと、繰り返した回数も取得できます
* タプルが帰ってきます

---

## ベクター

<pre><code data-source="snippets/vector/basics.rs" data-trim class="hljs rust"></code></pre>

* 可変長の配列（リスト）です
* vec! マクロで作成します
* 詳細は [Vec<T>](https://doc.rust-lang.org/stable/std/vec/) を参照してください
* [デモ](https://is.gd/XDOd0T)

----

### イテレーターを利用した繰り返し

<pre><code data-source="snippets/vector/scan.rs" data-trim class="hljs rust"></code></pre>

* iter メソッドで、ベクターをイテレーターに変換できます
* [デモ](https://is.gd/C6dys6)

----

### filter / map

<pre><code data-source="snippets/vector/filter-map.rs" data-trim class="hljs rust"></code></pre>

* イテレーターは filter や map メソッドを持っています
* [デモ](https://is.gd/fzmX9R)

---

## もくもくタイム：フィボナッチ数列

* [フィボナッチ数](https://ja.wikipedia.org/wiki/%E3%83%95%E3%82%A3%E3%83%9C%E3%83%8A%E3%83%83%E3%83%81%E6%95%B0)を計算する関数 `fib` を実装してみましょう
* [テンプレートはこちら](https://is.gd/7mKTCt) 

----

### ナイーブに JS で実装した例

<pre><code data-source="snippets/fib.js" data-trim class="hljs javascript"></code></pre>

---

## 所有権と move セマンティックス

<pre><code data-source="snippets/ownership.rs" data-trim class="hljs javascript"></code></pre>

* ビルドエラーとなります（[デモ](https://is.gd/Iy1MlJ))
* vec![1, 2, 3] の所有権が v から v2 へと移動してるためです
* 束縛するとは「値の所有権を持つ」ことと解釈されます

----

###　ビルドエラーにならない場合もあります 

<pre><code data-source="snippets/ownership-copy-trait.rs" data-trim class="hljs rust"></code></pre>

* i32 などは、束縛時に値をコピーします（[デモ](https://is.gd/iqsf3d))
* Copy trait を実装しているもの
* 上記の例では x と y は
    * 「同じ数値をもつ同じもの」を束縛しているのではなく
    * 「同じ数値をもつ、違うもの」を、それぞれ束縛しています

----

### 関数呼び出しと所有権　

<pre><code data-source="snippets/ownership-transfer-by-function-call.rs" data-trim class="hljs rust"></code></pre>

* ビルドエラーとなります（[デモ](https://is.gd/DocyHD))
* 一度目の関数呼び出しで、所有権が移動してしまうためです

----

### 所有権を返すように変更すれば OK

<pre><code data-source="snippets/ownership-restoring.rs" data-trim class="hljs rust"></code></pre>

* 所有権を返り値として返せば、ビルドエラーがおきません
* タプルを使えば、複数の値をまとめて返せます（[デモ](https://is.gd/eSwwXR))

---

## 参照

<pre><code data-source="snippets/reference.rs" data-trim class="hljs rust"></code></pre>

* ```&``` をつけることで、値への参照を取得できます（[デモ](https://is.gd/bJhCDQ))
* ```&型名```で、「その型への参照である」というアノテーションができます
* 参照を使うことで、値の貸し借りが行えます（borrowing）
* 関数呼び出しが終わると、所有権は自動的に元の所有者に戻ります

----

### 参照と変更可能性

<pre><code data-source="snippets/reference-immutable.rs" data-trim class="hljs rust"></code></pre>

* 参照している値の変更はできません
* ビルドエラーになります([デモ](https://is.gd/f8vsR1))

----

### 変更可能な参照

<pre><code data-source="snippets/reference-mutable.rs" data-trim class="hljs rust"></code></pre>

* ```&mut``` で変更可能な参照を取得できます
* ただし変更可能なオブジェクトに限ります

----

### 貸し借りのルール

* 参照専用の参照 (```&```) は複数作れます
* 変更可能な参照（```&mut```）は、1 つしか作れません
    * 状態を変化させられるのは、常に一人
    * 並列処理時の競合を避けるため

---

## もくもく：foo を実装してみましょう！

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

* 二つのベクトルの総和を求める関数 foo を実装してみましょう
* [テンプレートはこちら](https://is.gd/HtM4t1)
* 関数の型を変えてしまってもかまいません

---

## パターンマッチ

<pre><code data-source="snippets/match/basic.rs" data-trim class="hljs rust"></code></pre>

* match 式でパターンマッチが行えます（[デモ](https://is.gd/gXr0zW)）
* ```_``` は、カバーされなかったものにマッチします


----

### 複数のパターン

<pre><code data-source="snippets/match/union.rs" data-trim class="hljs rust"></code></pre>

* ```|``` を使うと、複数のパターンをまとめられます

----

### デストラクチャリング

<pre><code data-source="snippets/match/destructuring.rs" data-trim class="hljs rust"></code></pre>

* タプルのような構造を持ったものにもマッチできます

---

## 文字列

|型|説明|
|--|---|
|'static str|文字列リテラル|
|str|文字列のスライス|
|String|変更可能な文字列型|

----

### String と文字列の結合

<pre><code data-source="snippets/string/concatenation.rs" data-trim class="hljs rust"></code></pre>

* ```to_string``` メソッドで、String へ変換できます
* ```String + str``` もしくは ```String + &String``` で結合できます
* [デモ](https://is.gd/wFW5vf)

----

### str の作成

<pre><code data-source="snippets/str/slice.rs" data-trim class="hljs rust"></code></pre>

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

<pre><code data-source="snippets/cat/arguments.rs" data-trim class="hljs rust"></code></pre>

* [```std::env::args```](https://doc.rust-lang.org/std/env/fn.args.html) で引数をイテレーターとして取得できます
* collect メソッドで、イテレーターをベクターに変換しています
* 0 番目の要素は、実行されるコマンドの名前となっています

----

### コマンドライン引数付きで Cargo を実行するには

~~~sh
% cargo run -- src/main.rs
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/commandline-argument src/main.rs`
src/main.rs
~~~

* `--` オプションをつけた後に、プログラムに渡す引数を記述します
* 上記の例では、`src/main.rs` がプログラムへ渡されます

----

### ファイルの読み込み

<pre><code data-source="snippets/cat/read_file.rs" data-trim class="hljs rust"></code></pre>

* ```File::open```で ```File``` オブジェクト作り、ファイルにアクセスします
* 成功 / 失敗が判定できるように、```Result``` 型を返します

----

### 読み込んだファイルの出力

<pre><code data-source="snippets/cat/main.rs" data-trim class="hljs rust"></code></pre>

* 成功 / 失敗の判断をパターンマッチを使って行なっています
* ```Ok``` が成功時、```Err``` が失敗時です（```Result<T|E>```)
* () 内に変数で、成功 / 失敗時に取得される値を参照します

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

### 工夫できそうなこと

* 文字列が含まれない行の抜き出し
* 与えられた正規表現でのマッチ
* 並列化による高速化
* etc

---

## Have fun!

* 今回対象としなかったテーマ
    * ユーザー定義型（Struct / Enum）
    * ジェネリクス
    * trait, create, モジュール
    * キャスト
    * ライフタイム
    * 並列計算、クロージャー
* プルリクお待ちしています！[https://github.com/chikoski/rust-handson](https://github.com/chikoski/rust-handson) 
