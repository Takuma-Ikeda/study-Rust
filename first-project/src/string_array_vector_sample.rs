pub fn example() {
    /*
     * &str 型
     * 文字列リテラルを直接代入する場合に使う
     * 代入した値は原則変更できない
     */
    let str_slice: &str = "world";

    /*
     * String 型
     * from メソッドにより、 &str 型から String 型に変換している
     * String 型は値の変更が可能
     */
    let _string: String = String::from(str_slice);
    let _string_format: String = format!("Hello, {}", str_slice);

    /*
     * array 型: 固定長配列 -> 要素数を変更できない
     * mut キーワードを付けると要素の値を変更できるようになる
     */
    let mut array: [i32; 3] = [1, 2, 3]; // 要素数 3 で、i32 の値を持つ配列
    array[0] = 10;

    // 要素の追加はエラーになる
    // array.push(10);

    /*
     * Vector 型: 可変長配列 -> 要素数を変更できる
     * mut キーワードを付けると要素の値を変更できるようになる
     * vec![...] 記法: 配列と似た記述でベクタを定義できるように作られたマクロ -> Vec型オブジェクトを新規作成したのち 1、2、3 の各要素をベクタに追加するという処理が行われる
     */
    let mut vec: Vec<i32> = vec![1, 2, 3]; // 要素数 3 で、i32 の値を持つ配列
    vec[0] = 10;
    vec.push(10);
}
