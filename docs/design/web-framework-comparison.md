# Web Framework Comparison

## 共通仕様

- GET `/health` は `ok` を返す
- GET `/hello/{name}` は JSON で挨拶を返す

## 実装

- `projects/web_std_http`: 標準ライブラリのみで HTTP の基礎理解に集中
- `projects/web_axum`: ルーティングと extractors を使った軽量構成
- `projects/web_actix`: マクロベースの handler と高速サーバー構成

## ローカル実行

- `cargo run -p web_std_http`
- `cargo run -p web_axum`
- `cargo run -p web_actix`
