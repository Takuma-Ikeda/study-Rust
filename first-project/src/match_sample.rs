pub fn example() {
    let i = 5;

    /*
     * match 式
     * switch 文のようなもの
     * 構造体や列挙型の一部を変数に束縛して処理可能
     */
    match i {
        0 => println!("zero"),
        1 => println!("one"),
        // 複数条件
        2 | 3 => println!("two or three"),
        // 範囲指定
        4..=10 => println!("four to ten"),
        // どの条件にもマッチしない場合
        _ => println!("other"),
    }

    // match i 式の結果を変数 is_zero_str で束縛
    let is_zero_str = match i {
        0 => "zero",
        _ => "not zero",
    };

    // not zero が出力される
    println!("{}", is_zero_str);
}
