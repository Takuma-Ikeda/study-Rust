/**
 * struct (構造体)
 * 3 つの表現形態が存在する
 * 1. 名前付きのフィールドを持つ構造体
 * 2. 名前を付けずに複数の値を持つ構造体 (タプル構造体)
 * 3. 値を持たない構造体 (ユニット構造体)
 */

// 1. 名前付きのフィールドを持つ構造体
struct Fruit {
    name: String,
}

// impl キーワードの中でメソッド定義
impl Fruit {
    // インスタンスメソッドとして定義したい場合、&self という自身を指す引数を持たせる
    // 静的メソッドの扱い (関連関数) で定義したい場合、&self なしで定義する
    fn get_name(&self) -> &str {
        // 自身の構造体の名前付きフィールドにアクセス
        &self.name
    }
}

// 2. タプル構造体
struct Rectangle(i32, i32);

impl Rectangle {
    fn calc_area(&self) -> i32 {
        // 自身の構造体のタプル値にアクセス
        self.0 * self.1
    }
}

// ユニット構造体
struct Unit;

pub fn example() {
    let banana = Fruit {
        name: String::from("Banana"),
    };
    println!("{}", banana.get_name()); // Banana

    let rect = Rectangle(10, 20);
    println!("{}", rect.calc_area()); // 200

    let _unit = Unit;
}
