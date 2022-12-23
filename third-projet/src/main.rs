fn main() {
    println!("{}", div(4, 2));
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}

/*
 * #[test] アトリビュートを付けて関数を書くと cargo test でその関数だけ実行する
 */

#[test]
fn div_test() {
    assert_eq!(div(10, 3), 3);
}

/*
 * rust の仕様でゼロ除算を行うとクラッシュ (パニック) する
 * これをテストするには #[test] に加えて #[should_panic] アトリビュートを付ける
 */
#[test]
#[should_panic]
fn div_panic_test() {
    div(2, 0);
}

/**
 * cfg は config の略で、テスト時以外はコンパイルしないようにしてくれる
 * テストを書くときは別モジュール (mod) とするのが一般的
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_test() {
        assert_eq!(div(10, 3), 3);
    }
}
