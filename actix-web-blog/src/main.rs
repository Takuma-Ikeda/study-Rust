mod error;
mod repository;
mod schema;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{web, App, HttpResponse, HttpServer};
use error::ApiError;
use repository::{NewPost, Repository};

/*
 * エラーが起きる可能性がある以上、戻り値は Result 型にするのが適切
 * エラー発生時、適切な HttpResponse を ApiError 型から自動作成するトレイト (ResponseError) は error.rs 内に定義している
 * 引数
 * - repo: web::Data
 *   - main 関数内で登録したコネクションプールを呼び出している
 *   - web::Data 型は Arc と呼ばれる参照カウントを使ったスマートポインタの拡張型。Repository 型データ (コネクションプール) を各 POST リクエスト処理で共有できるようになる
 * - new_post: web::Json<NewPost>
 *   - リクエストボディの JSON を NewPost 型にデシリアライズした値
 */
#[actix_web::post("/posts")]
async fn create_post(
    repo: web::Data<Repository>,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let new_post = new_post.into_inner();

    // バリデーション
    if !new_post.validate() {
        return Ok(HttpResponse::BadRequest().body("length of tilte is invalid."));
    }

    // 結果は Result 型であり、エラーが起きる可能性があるので末尾に ? をつける
    let post = repo.create_post(new_post).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::get("/posts")]
async fn list_posts(repo: web::Data<Repository>) -> Result<HttpResponse, ApiError> {
    let res = repo.list_posts().await?;
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::get("/posts/{id}")]
async fn get_post(
    repo: web::Data<Repository>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    // GET パラメータ {id} を取得する
    let id = path.into_inner();
    let res = repo.get_post(id).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ミドルウェアを使えるようにする
    tracing_subscriber::fmt::init();

    // 環境変数から URL を取り出し repo を作る処理
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let repo = web::Data::new(Repository::new(&database_url));

    // repo の登録処理
    // app_data(repo.clone()) の箇所で、変数 repo を他のアクションメソッド (service) でも使えるコネクションプールとして登録している
    HttpServer::new(move || {
        App::new()
            .app_data(repo.clone())
            .wrap(Logger::default()) // ミドルウェア Logger 追加
            .wrap(NormalizePath::trim()) // ミドルウェア NormalizePath 追加
            .service(create_post)
            .service(list_posts)
            .service(get_post)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
