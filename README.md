# study-Rust

## Docs

- https://rust-jp.rs/

## 環境構築

- https://www.rust-lang.org/ja/tools/install
  - 公式手順

```sh
# 選択肢は default を選ぶ
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### PATH を通す

zshenv の場合、`~/.zshenv` に下記追加

```sh
export RUST_PATH=~/.cargo/bin
export PATH="$RUST_PATH:$PATH"
```

```sh
source ~/.zshenv

# Rust バージョン確認
rustc --version
rustc 1.66.0 (69f9c33d7 2022-12-12)
```

## cargo コマンド

```sh
# first-project/src/main.rs という Hello World を標準出力する雛形ファイルが作成される
cargo new first-project
cd first-project

# main.rs が実行される
cargo run

# コードの自動整形
cargo fmt

# コードの静的解析
cargo clippy
```

## cargo make コマンド

```sh
# first-project/Makefile.toml に定義した watch コマンドを実行
cargo make watch
```
