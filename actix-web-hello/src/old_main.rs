/*
 * use 行
 * プログラム内で使う型を指定する
 * ここでは actix_web クレードから必要な型をインポートしている
 */
use actix_web::{http::header::ContentType, App, HttpResponse, HttpServer};

/*
 * リクエストして呼び出される関数を定義
 * [actix_web::get("/")] の部分はアトリビュートマクロ
 */
#[actix_web::get("/")]
async fn hello() -> HttpResponse {
    // Content-Type ヘッダと JSON 文字列を含む HTTP レスポンスを戻り値とする
    // r#"～"# は文字列リテラルの記述方法の一つ: エスケープなしにダブルクオートの文字列を書ける
    HttpResponse::Ok()
        .append_header(ContentType::json())
        .body(r#"{"greet":"Hello, world!"}"#)
}

/*
 * main関数
 * #[actix_web::main] アトリビュート: Actix Web 独自の非同期ランタイム
 * HttpServer インスタンスを作成する: Web アプリケーションの機能定義 (App) を引数にとる
 * 自身の IP アドレスとポートを指定して実行する
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // bind メソッドのあとの ? は Rust のエラー処理の糖衣構文
    // bind はポートが他で使用中の際などに失敗する可能性があるため Result<T, E> 型の値を返す
    // そこで ? を適用すると「失敗時 (Err(E)) は ? の箇所で return して、成功時 (Ok(T)) は中身 (T) を取り出す」動作をできる
    // ? が使えるのは Result<T, E> を返す関数内に限られる
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}
