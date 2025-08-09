# AuthorizedUser（`AuthenticatedUser`）の使い方

本プロジェクトでは、ハンドラ関数に「認証済みユーザー」を型で受け取れるように、Actix Web の `FromRequest` を実装した `AuthenticatedUser` を提供しています。本ドキュメントでは、一般的な呼称として AuthorizedUser と記載しますが、コード上の型名は `AuthenticatedUser` です（同義）。

## できること
- **JWT からユーザー情報（`JWTClaim`）を抽出**し、ハンドラ引数として受け取れます。
- ハンドラ内では `user.0` として `JWTClaim` にアクセスできます。

## 事前準備（アプリケーション設定）
`JwtTokenService` を `App::app_data` に登録しておきます。`main.rs` では既に次のように設定されています。

```rust
use actix_web::{web, App, HttpServer};
use middleware::auth::token::JwtTokenService;

HttpServer::new(move || {
    let jwt = web::Data::new(JwtTokenService::from_env().expect("JWT config"));
    App::new()
        .app_data(jwt.clone())
        // .service(...) など
})
```

環境変数:
- `JWT_SECRET`: 署名鍵（必須）
- `JWT_EXP_SECS`: 有効期限（省略時は 3600 秒）

## 使い方（ハンドラ）
`AuthenticatedUser` を引数に追加するだけで、認証済みユーザーのクレームを取得できます。

```rust
use actix_web::{get, HttpResponse, Responder};
use crate::middleware::auth::extractor::AuthenticatedUser;

#[get("/me")]
pub async fn me(user: AuthenticatedUser) -> impl Responder {
    // `user.0` が `JWTClaim`（sub, iat, exp）
    HttpResponse::Ok().json(user.0)
}
```

`JWTClaim` の構造:

```rust
pub struct JWTClaim {
    pub sub: i64, // ユーザーID
    pub iat: i64, // 発行時刻
    pub exp: i64, // 期限
}
```

## リクエスト要件（クライアント側）
- HTTP ヘッダー `Authorization: Bearer <JWT>` を付与してください。
- トークンは `POST /auth/login` のレスポンス（`{ token: string }`）から取得できます。

curl 例:

```bash
# ログインしてトークン取得
TOKEN=$(curl -s -X POST http://localhost:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"user@example.com","password":"password"}' | jq -r .token)

# 認証付きで /me を叩く
curl -i http://localhost:8080/me -H "Authorization: Bearer ${TOKEN}"
```

## エラー挙動（ハンドラに到達する前に 401）
`AuthenticatedUser` は `FromRequest` 実装により、次の条件で 401 を返します。
- `Authorization` ヘッダーがない、もしくは `Bearer` 形式でない
- トークンが不正、または期限切れ
- `JwtTokenService` が未登録（アプリ設定ミス）

## テストのヒント
- アプリ内のログインハンドラを使う（推奨）
- もしくは `JwtTokenService::generate(user_id)` でトークンを自前生成し、`Authorization` に付与

## 実装ファイル
- エクストラクタ: `src/middleware/auth/extractor.rs`（`AuthenticatedUser` / `FromRequest`）
- クレーム: `src/middleware/auth/model.rs`（`JWTClaim`）
- トークンサービス: `src/middleware/auth/token.rs`（`JwtTokenService`）


