# Monorepo Design

## 目的

- Rust の基礎学習と Web アプリ開発を同時に進める
- 小さい学習 crate と実践 project を同一 workspace で管理する

## 方針

- 学習用 crate は独立性を優先し、相互依存は避ける
- 実践 project は最小機能から始め、必要時に共通ロジックを crate 化する
- CI で `fmt`、`clippy`、`test` を必ず実行する
