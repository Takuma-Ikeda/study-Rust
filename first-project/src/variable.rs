pub fn example() {
    // 型なしの let x = 100; でも型推論される
    let x: i32 = 100;

    // imutable なので、再代入不可
    // x = 100;

    // mut で mutable になるので、再代入できる
    let mut y = 50;

    // 文字列にしないと println できない
    println!("{}", y);

    y = 300;

    println!("{}", x);
    println!("{}", y);
}
