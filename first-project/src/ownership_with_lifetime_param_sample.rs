/*
 * ライフタイムパラメータ 'a の利用
 */

struct Greet<'a> {
    // 参照している値がすぐにメモリ開放されると困るので、ライフタイムパラメータ a' で明示する
    word: &'a str,
}

impl<'a> Greet<'a> {
    fn say(&self) {
        println!("{}", self.word);
    }
}

pub fn example() {
    // &str 型なので参照
    let hello: &str = "Hello!";
    {
        // Greet のフィールド word に &str の値を渡す
        let greet = Greet { word: hello };
        greet.say();
    } // 変数 greet のメモリはここで解放される

    println!("{}", hello);
} // 変数 hello のメモリはここで解放される
