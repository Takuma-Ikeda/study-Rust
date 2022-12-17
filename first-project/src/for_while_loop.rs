pub fn example() {
    /*
     * for ... in ...
     * in の後には配列・ベクターのようなコレクションを置くことができる
     */
    for i in 0..10 {
        println!("in for-loop: {}", i);
    }

    /*
     * while
     */
    let mut count = 0;
    while count < 10 {
        // count++ のような表記はできないので注意
        count += 1;
    }

    /*
     * loop
     * 条件が常に true の while のようなもの
     */
    loop {
        count -= 1;
        if count == 0 {
            break;
        }
    }
}
