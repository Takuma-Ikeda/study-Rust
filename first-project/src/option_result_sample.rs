/*
 * enum による Option 型定義
 * <T> はジェネリック型であり、任意の型を当てはめることができる
 * Option<i32> であれば Some(10)
 * Option<Color> であれば Some(Color::Custom(10, 123, 255))
 */
enum Option<T> {
    None,
    Some(T),
}

/*
 * enum による Result 型定義
 * エラーを表現する <E> のジェネリック型も当てはめることができる
 * 関数の戻り値として、処理成功時には Ok(T) を返し、エラー時は Err(E) でエラー事由を返す…という使い方をする
 */
enum Result<T, E> {
    Ok(T),
    Err(E),
}
