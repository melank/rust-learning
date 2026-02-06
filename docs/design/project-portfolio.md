# Project Portfolio

このモノレポでは、各 `projects/*` が異なる課題を担当します。

## projects/api_service

- 学習テーマ: Web API / DB / migration
- 想定スタック: axum, sqlx, sqlite
- ゴール: CRUD API と簡易認証

## projects/worker_queue

- 学習テーマ: 非同期処理 / 再試行 / バックプレッシャー
- 想定スタック: tokio, channel, tracing
- ゴール: ジョブの投入、処理、失敗管理

## projects/cli_automation

- 学習テーマ: CLI 設計 / ファイル処理 / 運用スクリプト代替
- 想定スタック: clap, serde, anyhow
- ゴール: 実務的な自動化CLIを1本完成

## projects/tui_dashboard

- 学習テーマ: TUI / イベントループ / 状態管理
- 想定スタック: ratatui, crossterm
- ゴール: ローカル監視ダッシュボード

## projects/web_frontend

- 学習テーマ: Rustフロントエンド / 状態管理 / API連携
- 想定スタック: leptos or yew
- ゴール: API連動する SPA の最小実装
