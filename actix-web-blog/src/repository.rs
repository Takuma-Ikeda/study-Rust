use crate::error::ApiError;
use crate::schema::*;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use serde::Deserializer;
use serde::{Deserialize, Serialize};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Repository {
    pool: DbPool,
}

/*
 * Post リクエスト時に受け取る型
 * Diesel が NesPost 型であっても posts テーブルに INSERT できるように紐付けている
 * #[serde(deserialize_with = "max100")] でデシリアライズ時、関数 max100 を使って自動的にバリデーションするようにしている
 */
#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    #[serde(deserialize_with = "max100")]
    title: String,
    body: String,
}

/*
 * 明示的に実行する場合のバリデーションメソッド
 */
impl NewPost {
    pub fn validate(&self) -> bool {
        self.title.len() > 0 && self.title.len() <= 100
    }
}

/*
 * デシリアライズ時、自動的に実行する場合のバリデーションメソッド
 */
fn max100<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(de).and_then(|s| {
        if !s.is_empty() && s.len() <= 100 {
            Ok(s)
        } else {
            Err(serde::de::Error::custom("string length is 0 or too long."))
        }
    })
}

/*
 * レコードの全フィールドを持たせる型
 * Diesel がオブジェクトとして DB レコード保持できるように Queryable を derive している
 */
#[derive(Serialize, Queryable)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

impl Repository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create a pool."); // expect はエラーが起きたらアプリを強制終了 (パニック) させる: DB 接続時エラーを考慮
        Self { pool }
    }

    /*
     * NewPost 型を受け取り、DB Insert 成功時には Post 型を返却する
     * エラー時には ApiError 型の値を返却する: そのタイミングは ? が書かれている箇所
     * ※ From トレイトの実装により ? の記述だけで早期 return できるようになっている
     */
    pub async fn create_post(&self, new_post: NewPost) -> Result<Post, ApiError> {
        let mut conn = self.pool.get()?; // エラーがあれば ApiError を返却

        /*
         * web::block とは
         * create_post は非同期メソッドとして定義したが insert_into は同期的メソッドである
         * そのまま insert_into を使うと処理をブロッキングするので、create_post を非同期メソッドとして定義した意味が薄れてしまう
         * このようなブロッキングしてしまう入出力処理を async メソッド内で使いたい場合、
         * Actix Web は起動時に用意したスレッドプールで実行できる機能を web::block として提供している
         * ただし、別スレッドに引数のクロージャ (move || { ～ }) を移すため、参照 (借用) ではデータを受け渡せない
         * また new_post の所有権はクロージャにムーブするため web::block 以降は new_post が使えなくなる点に注意する
         */
        let post = web::block(move || {
            diesel::insert_into(posts::table)
                .values(new_post)
                .get_result(&mut conn)
        })
        .await??; // エラーがあれば ApiError を返却...を 2 回適用しているだけ

        Ok(post)
    }

    pub async fn list_posts(&self) -> Result<Vec<Post>, ApiError> {
        let mut conn = self.pool.get()?;
        let res = web::block(move || posts::table.load(&mut conn)).await??;

        Ok(res)
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, ApiError> {
        let mut conn = self.pool.get()?;
        let res = web::block(move || posts::table.find(id).first(&mut conn).optional())
            .await??
            .ok_or(ApiError::NotFound)?;

        Ok(res)
    }
}

/*
 * DB 接続テスト
 * Rust は同じファイルにテストを書くことができる
 * cargo make --env-file .env watch 実行中であれば、テストも自動的に実行される
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conn() {
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let repo = Repository::new(&database_url);
        assert!(repo.pool.get().is_ok());
    }
}
