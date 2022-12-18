/*
 * trait
 * Java でいう interface、継承・ポリモフィズムのようなもの
 */
trait Greeter {
    // この trait を使用するときは greet を実装しないといけない
    fn greet(&self);
}

struct Person(String);

// for で利用したい他の構造体を指定できる
impl Greeter for Person {
    fn greet(&self) {
        println!("Hello, I am {}!", self.0); // self.0 は Person(String) の String を参照する
    }
}

/*
 * 型定義の真上で attribute というものを宣言できる
 * attribute はマクロの一種: コンパイラがその型用の実装を自動的に導出する
 * 標準的な実装とは異なる実装をしたい場合、impl で自ら実装することもできる
 *
 * #[derive(XXX)] は型に標準的 (汎用的な) な実装を追加するトレイト
 * 以下では Hours 型にデバッグ出力機能を付加している
 */
#[derive(Debug)]
struct Hours(u32);

pub fn example() {
    let h = Hours(5);

    // デバッグ機能
    println!("{:?}", h); // => Hours(5)
}
