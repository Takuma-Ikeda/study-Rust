// ランタイムクレートは main 関数をアトリビュートマクロでラップすると、fn main() の関数の形に変換して非同期処理を実行する
#[tokio::main]
pub async fn example() -> std::io::Result<()> {
    let _ = tokio::fs::read("file.txt").await?;
    Ok(())
}
