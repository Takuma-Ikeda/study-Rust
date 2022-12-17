pub fn example() {
    let x = 100;
    let y = 50;

    if x == y {
        println!("same value!");
    }

    // 三項演算子風
    // if 式の評価結果を変数に束縛している
    let _z = if x != y { 500 } else { 300 };
}
