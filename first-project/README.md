# first-project

## Notion

https://www.notion.so/eeeeg/Rust-Tips-1cec62b82452481e91a52dd63903da6e#71c1a0ad84da43ea8f101cb5150bf24a

## 非同期処理のランタイム環境

今回は [tokio](https://tokio.rs/) を利用する

```sh
# Tokio 学習が第一目標となるように設計されたライブラリ
# ソースコード中にもたくさんのコメントがある
# 現実の Redis ライブラリに求められる機能がいくつか欠けているので商用では使わないほうがいい
cargo install mini-redis

# サーバ立ち上げ
mini-redis-server

# (nil) が表示されたら OK
mini-redis-cli get foo
```
