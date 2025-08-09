# メモ帳アプリ (Rust / Actix Web / SQLite)

シンプルなメモ帳アプリです。

## 機能

- ユーザー登録（サインアップ）
- ログイン（JWT認証）
- 自分のユーザー情報取得
- メモの作成
- メモの取得（公開: 誰でも閲覧可能）
- メモの更新（作成者のみ可能）

## セットアップ

### 必要なもの
- Rust/cargo
- SQLite
- slqx (for developers)

### 手順:
1. 依存関係の取得
   ```bash
   cargo install
   ```
1. 環境変数の設定（例）
   ```bash
   export DATABASE_URL=sqlite:memo.db
   export JWT_SECRET=secret-jwt
   export JWT_EXP_SECS=86400
   ```
1. 実行
   ```bash
   cargo run
   ```

## 開発者向け
- [マイグレーション手順](docs/migrations.md)
- [AuthorizedUser（AuthenticatedUser）の使い方](docs/authorized_user.md)
