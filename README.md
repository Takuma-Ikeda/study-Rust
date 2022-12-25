# study-Rust

## Notion

https://www.notion.so/eeeeg/Rust-Tips-1cec62b82452481e91a52dd63903da6e

## Docs

- https://rust-jp.rs/

## Rust 環境構築

- https://www.rust-lang.org/ja/tools/install
  - 公式手順
- https://doc.rust-lang.org/cargo/
  - Cargo
- https://crates.io/
  - Crate.io
- https://lib.rs/
  - Lib.rs

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
