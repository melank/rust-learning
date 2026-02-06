# rust-learning

Rust Web アプリ開発を通じて学習するためのモノレポです。

## Workspace 構成

```text
/
├─ crates/
│  ├─ w01_basics/
│  ├─ w02_ownership/
│  ├─ w03_error_handling/
│  ├─ w04_traits/
│  └─ w05_iterators/
├─ projects/
│  ├─ cli_tool/
│  ├─ web_std_http/
│  ├─ web_axum/
│  ├─ web_actix/
│  ├─ api_service/
│  ├─ worker_queue/
│  ├─ cli_automation/
│  ├─ tui_dashboard/
│  └─ web_frontend/
├─ docs/
│  ├─ notes/
│  └─ design/
└─ .github/workflows/ci.yml
```

## 使い方

- 全体テスト: `cargo test --workspace`
- フォーマット: `cargo fmt --all`
- Lint: `cargo clippy --workspace -- -D warnings`

## 学習方針

- `crates/` は週ごとのテーマを切り出して検証する
- `projects/` はそれぞれ異なる課題を持つ実践プロジェクトとして運用する
- 共通化は必要になったタイミングで `crates/` に切り出す

## プロジェクト活用

- 各プロジェクトの狙い: `docs/design/project-portfolio.md`
- Web比較メモ: `docs/design/web-framework-comparison.md`
- 週次メモ: `docs/notes/`
