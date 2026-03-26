mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;

use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::AppConfig;
use routes::api_router;

/// Shared application state injected into every request handler.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "create-admin" {
        let config = AppConfig::from_env();
        let pool = connect_to_database(&config.database_url).await;
        run_create_admin(&args[2..], &pool).await;
        return;
    }

    let config = AppConfig::from_env();
    let pool = connect_to_database(&config.database_url).await;

    let state = AppState { pool, config: config.clone() };
    let app = api_router(state)
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http());

    let address = config.socket_address();
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to {address}: {e}"));

    tracing::info!("Blog API listening on http://{address}");
    axum::serve(listener, app).await.expect("Server crashed");
}

async fn run_create_admin(args: &[String], pool: &PgPool) {
    use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2};
    use repositories::user_repository::UserRepository;

    let mut username = None;
    let mut email = None;
    let mut password = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--username" => { username = args.get(i + 1).map(|s| s.as_str()); i += 2; }
            "--email"    => { email    = args.get(i + 1).map(|s| s.as_str()); i += 2; }
            "--password" => { password = args.get(i + 1).map(|s| s.as_str()); i += 2; }
            _            => { i += 1; }
        }
    }

    let (username, email, password) = match (username, email, password) {
        (Some(u), Some(e), Some(p)) => (u, e, p),
        _ => {
            eprintln!("Usage: blog-backend create-admin --username <name> --email <email> --password <password>");
            std::process::exit(1);
        }
    };

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let repo = UserRepository::new(pool);
    match repo.create_admin(username, email, &password_hash).await {
        Ok(user) => println!("Admin '{}' created with id {}", user.username, user.id),
        Err(e)   => { eprintln!("Error: {e}"); std::process::exit(1); }
    }
}

async fn connect_to_database(url: &str) -> PgPool {
    PgPool::connect(url)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to database: {e}"))
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
