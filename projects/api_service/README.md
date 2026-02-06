# api_service

`axum + sqlx + sqlite + tokio` で構築する API プロジェクトです。

## 初手ディレクトリ構成

```text
projects/api_service/
├─ migrations/
│  └─ 0001_create_notes.sql
└─ src/
   ├─ app.rs
   ├─ db.rs
   ├─ error.rs
   ├─ models.rs
   ├─ routes.rs
   └─ main.rs
```

## 役割

- `main.rs`: サーバー起動
- `app.rs`: 初期化
- `db.rs`: DB接続と migration
- `routes.rs`: `/health`, `/notes`
- `models.rs`: request/response, DB model
- `error.rs`: APIエラー

## エンドポイント

- `GET /health`
- `GET /notes`
- `POST /notes`

## 実行

```bash
cargo run -p api_service
```

環境変数:

- `PORT` (default: `8083`)
- `DATABASE_URL` (default: `sqlite://data/api_service.db`)

詳細: `docs/design/api-service-ecosystem.md`
