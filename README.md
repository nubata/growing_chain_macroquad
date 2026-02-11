# Growing chain macroquad
Macroquad crateを用いて実装された、成長するチェーンのウェブインスタレーション。

## ビルド方法
Rust (Rustup)をインストールして以下を実行。

```shell
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --profile release
```

## 実行方法
以下を実行して、表示されたURLをブラウザで開く。

```shell
cargo install basic-http-server
basic-http-server .
```
