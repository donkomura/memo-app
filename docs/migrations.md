# マイグレーション手順（SQLite / SQLx）

このプロジェクトは SQLite を使用します。接続文字列は `DATABASE_URL` で指定します（例: `sqlite:db.sqlite`）。

## 事前準備
- sqlx-cli をインストール

```bash
# 推奨
cargo install sqlx-cli --version ^0.8 --no-default-features --features sqlite
```

## 基本コマンド

```bash
# DB 作成（必要時）
sqlx database create

# 既存マイグレーション適用
sqlx migrate run --source db/migrations

# 状態確認
sqlx migrate info --source db/migrations

# 直前を取り消し
sqlx migrate revert --source db/migrations

# 新規マイグレーション追加
sqlx migrate add -r <migration_name> --source db/migrations
```

## よくある操作

```bash
# DB を作り直して再適用
rm -f db.sqlite
sqlx database create
sqlx migrate run --source db/migrations
```
