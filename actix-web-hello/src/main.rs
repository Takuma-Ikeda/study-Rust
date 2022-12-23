use actix_web::{web, App, HttpResponse, HttpServer};

// Serde の型をインポート
use serde::{Deserialize, Serialize};

/**
 * リクエストのクエリに対応する型
 * derive アトリビュートで Deserialize を指定
 * -> Deserialize を derive する、という
 * クエリの同名のフィールドに指定された値が格納されるようになる
 */
#[derive(Deserialize)]
struct HelloQuery {
    name: String,
    age: u32,
}

/*
 * レスポンスの JSON 構造に対応する型
 * Serialize を derive しており JSON に変換できる
 */
#[derive(Serialize)]
struct HelloResponse {
    greet: String,
}

#[actix_web::get("/")]
async fn hello(
    query: web::Query<HelloQuery>, // リクエストパラメータを仮引数で受け取る: 型にマッチしないと 400 (Bad Request) を即座に返す
) -> HttpResponse {
    // デシリアイズしてクエリから値を取り出す: HelloQuery の同名フィールドに自動的に格納される
    let query = query.into_inner();
    let message = format!(
        "Hello, my name is {}! I am {} years old!",
        query.name, query.age
    );
    let h = HelloResponse { greet: message };

    // JSON レスポンスとしてシリアライズする: Content-Type のヘッダーまで設定してくれる
    HttpResponse::Ok().json(h)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}
