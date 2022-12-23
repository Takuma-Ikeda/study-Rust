/*
 * enum (列挙型) による Color 型定義
 * 構造体を定義できる
 */
enum Color {
    Red,                // ユニット構造体
    Green,              // ユニット構造体
    Blue,               // ユニット構造体
    Custom(u8, u8, u8), // タプル構造体
}

impl Color {
    fn color_code(&self) -> String {
        // インスタンスの enum の型で self の型でパターンマッチする
        match self {
            Color::Red => String::from("#ff0000"),
            Color::Green => String::from("#00ff00"),
            Color::Blue => String::from("#0000ff"),
            Color::Custom(r, g, b) => {
                format!(
                    // 引数で受け取った数値を 2 桁の 16 進数で出力する
                    "#{:02x}{:02x}{:02x}",
                    r, g, b
                )
            }
        }
    }
}

pub fn example() {
    let color = Color::Blue;
    println!("{}", color.color_code()); // #0000ff

    let color = Color::Custom(10, 123, 255);
    println!("{}", color.color_code()); // #0a7bff
}
