# Growing chain macroquad
Macroquad crateを用いて実装された、成長する鎖のインタラクティブデモ。

[Web上で閲覧する](https://nubata.github.io/growing_chain_macroquad/)

## 事前準備
以下をインストールする。

- Rust (Rustup)
- GNU Make

## ビルド方法
以下を実行。

```shell
rustup target add wasm32-unknown-unknown
make build
```

## 実行方法
以下を実行して、表示されたURLをブラウザで開く。

```shell
make run
```
