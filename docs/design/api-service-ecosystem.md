# api_service Ecosystem Guide

## 最小依存セット

- `tokio`: async ランタイム、TCP リスナー、非同期ファイル I/O
- `axum`: ルーティング、JSON入出力、状態共有
- `sqlx`: SQLite への非同期クエリ実行、pool、migration
- `sqlite`: 開発初期に扱いやすいファイル DB
- `serde`: request/response のシリアライズ
- `thiserror`: API 用エラー型を簡潔に定義

## 依存の関係

`tokio` の上で `axum` が動き、`axum` の handler 内から `sqlx` で `sqlite` にアクセスします。

## ディレクトリ構成の狙い

- `src/main.rs`
  - プロセス起動、ポート設定、HTTPサーバー起動だけに責務を限定
- `src/app.rs`
  - アプリ初期化（DB接続、router 組み立て）
- `src/db.rs`
  - DB pool の生成と migration 実行
- `src/routes.rs`
  - endpoint と handler の定義
- `src/models.rs`
  - request/response と DB row モデル
- `src/error.rs`
  - エラーを HTTP レスポンスへ変換
- `migrations/`
  - スキーマの変更履歴を SQL で管理

## 初手で学べるポイント

- async サーバーの起動フロー
- 状態共有（`State`）の基本
- SQLite migration 運用
- バリデーションと API エラー設計
