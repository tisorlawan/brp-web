use crate::brp::model::UserReadings;
use crate::errors::ApiError;
use crate::view::hx::{HxHeaderBuilder, HxSwap};
use crate::AppState;
use axum::extract::{FromRequestParts, Query, State};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Redirect};
use axum::Extension;
use axum_extra::extract::PrivateCookieJar;
use chrono::{Duration, Local};
use cookie::Key;
use lazy_static::lazy_static;
use oauth2::reqwest::async_http_client;
use oauth2::{basic::BasicClient, AuthUrl, TokenUrl};
use oauth2::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse};
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use sqlx::query;
use tower_sessions::cookie::Cookie;

lazy_static! {
    pub static ref GOOGLE_OAUTH_CLIENT_ID: String = std::env::var("GOOGLE_OAUTH_CLIENT_ID")
        .expect("GOOGLE_OAUTH_CLIENT_ID env var must be set");
    pub static ref GOOGLE_OAUTH_CLIENT_SECRET: String = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
        .expect("GOOGLE_OAUTH_CLIENT_SECRET env var must be set");
    pub static ref GOOGLE_OAUTH_REDIRECT_URL: String = std::env::var("GOOGLE_OAUTH_REDIRECT_URL")
        .expect("GOOGLE_OAUTH_REDIRECT_URL env var must be set");
    pub static ref GOOGLE_OAUTH_SCOPE: String =
        std::env::var("GOOGLE_OAUTH_SCOPE").expect("GOOGLE_OAUTH_SCOPE env var must be set");
    pub static ref JWT_SECRET: String =
        std::env::var("JWT_SECRET").expect("JWT_SECRET env var must be set");
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserCallback {
    pub email: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub email: String,
    pub id: i64,
}

#[axum::async_trait]
impl FromRequestParts<AppState> for Option<User> {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookiejar = PrivateCookieJar::<Key>::from_request_parts(parts, state).await?;
        let Some(cookie) = cookiejar.get("sid").map(|cookie| cookie.value().to_owned()) else {
            return Ok(None);
        };

        let res = sqlx::query_as::<_, User>(
            "SELECT
                users.email, users.id
            FROM sessions
            LEFT JOIN USERS ON sessions.user_id = users.id
            WHERE sessions.session_id = $1
            LIMIT 1",
        )
        .bind(cookie)
        .fetch_one(&state.db)
        .await;

        match res {
            Ok(res) => Ok(Some(User {
                email: res.email,
                id: res.id,
            })),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(e.into()),
            },
        }
    }
}

pub fn build_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    let redirect_url = "http://localhost:3000/api/auth/google_callback".to_string();
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

pub async fn google_callback(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>,
) -> Result<impl IntoResponse, ApiError> {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .unwrap();

    let ctx = ReqwestClient::new();

    let profile = ctx
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret().to_owned())
        .send()
        .await
        .unwrap()
        .json::<UserCallback>()
        .await
        .unwrap();

    let Some(secs) = token.expires_in() else {
        todo!()
    };

    let secs: i64 = secs.as_secs().try_into().unwrap();

    let max_age = Local::now().naive_local() + Duration::try_seconds(secs).unwrap();
    let cookie = Cookie::build(("sid", token.access_token().secret().to_owned()))
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(secs))
        .build();

    let row = sqlx::query("INSERT INTO users (email) VALUES ($1) ON CONFLICT (email) DO NOTHING")
        .bind(profile.email.clone())
        .execute(&state.db)
        .await
        .unwrap();
    if row.rows_affected() == 1 {
        // new user added
        UserReadings::new_with_default_readings(row.last_insert_rowid())
            .replace_current(&state.db)
            .await;
    }

    let session = token.access_token().secret().to_owned();
    sqlx::query!(
        "INSERT INTO sessions (user_id, session_id, expires_at) VALUES (
            (SELECT id FROM USERS WHERE email = $1 LIMIT 1),
            $2,
            $3
        )
        ON CONFLICT (user_id)
            DO UPDATE SET
            session_id = excluded.session_id,
            expires_at = excluded.expires_at",
        profile.email,
        session,
        max_age
    )
    .execute(&state.db)
    .await
    .unwrap();

    Ok((jar.add(cookie), Redirect::to("/")))
}

pub async fn post_logout(State(state): State<AppState>, user: Option<User>) -> impl IntoResponse {
    if let Some(user) = user {
        query!("DELETE FROM sessions WHERE user_id = ?", user.id)
            .execute(&state.db)
            .await
            .unwrap();
        return HxHeaderBuilder::new()
            .with_swap(HxSwap::None)
            .with_redirect("/login")
            .build();
    }

    // Handler Error
    HxHeaderBuilder::new()
        .with_swap(HxSwap::None)
        .with_redirect("/login")
        .build()
}
