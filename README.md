# メモ帳アプリ (Rust / Actix Web / SQLite)

シンプルなメモ帳アプリです。Actix Web + SQLx + SQLite を採用し、リポジトリ層で永続化処理を分離しています。

## セットアップ
- 必須: Rust/cargo, SQLite
- 手順:
  1. 依存関係の取得
     ```bash
     cargo build
     ```
  2. 環境変数の設定（例）
     ```bash
     echo "DATABASE_URL=sqlite:memo.db" > .env
     ```

## 開発者向け
- [マイグレーション手順](docs/migrations.md)
