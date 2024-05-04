use axum::{
    handler::Handler,
    routing::{get, post},
    Extension, Router,
};
use brp_web::{
    auth::{self, GOOGLE_OAUTH_CLIENT_ID, GOOGLE_OAUTH_CLIENT_SECRET},
    brp,
    view::pages::login,
    AppState,
};
use cookie::Key;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
// #[cfg(debug_assertions)]
// use tower_livereload::LiveReloadLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    println!("PID: {}", std::process::id());
    dotenv::dotenv().expect("successfully load .env");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "brp_web=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let sqlite_pool = SqlitePoolOptions::new()
        .connect("./database.sqlite")
        .await
        .unwrap();

    let state = AppState {
        db: sqlite_pool,
        key: Key::from(
            &hex::decode(auth::JWT_SECRET.as_str())
                .expect("valid hex string with minimum bytes 64"),
        ),
    };
    let brp_router = Router::new()
        .route(
            "/",
            get(brp::page_brp.layer(Extension(GOOGLE_OAUTH_CLIENT_ID.as_str()))),
        )
        .route("/dates", post(brp::post_dates))
        .route("/q", get(brp::get_q_chapter));

    let router = Router::new()
        .nest("/", brp_router)
        .route("/login", get(login::page_login))
        .route("/logout", post(auth::post_logout))
        .layer(TraceLayer::new_for_http())
        .route("/api/auth/google_callback", get(auth::google_callback))
        .nest_service(
            "/static",
            ServeDir::new("./assets/dist/").precompressed_gzip(),
        )
        .layer(Extension(auth::build_oauth_client(
            GOOGLE_OAUTH_CLIENT_ID.to_string(),
            GOOGLE_OAUTH_CLIENT_SECRET.to_string(),
        )))
        .with_state(state);

    // #[cfg(debug_assertions)]
    // let router = router.layer(LiveReloadLayer::new());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
