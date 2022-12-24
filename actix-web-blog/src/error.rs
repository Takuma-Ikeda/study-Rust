use actix_web::{HttpResponse, ResponseError};

/*
 * 列挙型で下記エラーを ApiError として定義
 * - 対象データが存在しない (404 NotFound)
 * - それ以外 (503 Service Temporary Unavaliable)
 */
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Post not found")]
    NotFound,
    #[error(transparent)]
    Other(anyhow::Error),
}

/*
 * macro_rules!
 * 定義の記述を簡略化できる関数的マクロ
 * 今回は、利用するクレートで発生する可能性があるエラー型は ApiError::Other に変換してしまう From トレイトを実装している
 * From トレイト自体は型変換に使う Rust 標準のトレイト
 * エラー処理に ? を使っていると、From は自動適用されるようになる
 */
macro_rules! impl_from_trait {
    ($etype: ty) => {
        impl From<$etype> for ApiError {
            fn from(e: $etype) -> Self {
                ApiError::Other(anyhow::anyhow!(e))
            }
        }
    };
}

// 下記エラーが発生しても ApiError::Other として変換される
impl_from_trait!(diesel::r2d2::Error);
impl_from_trait!(diesel::r2d2::PoolError);
impl_from_trait!(diesel::result::Error);
impl_from_trait!(actix_web::error::BlockingError);

/*
 * ResponseError は発生したエラーごとに適切な HttpResponse を自動生成するトレイト
 */
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().finish(),
            ApiError::Other(_) => HttpResponse::ServiceUnavailable().finish(),
        }
    }
}
