use std::env;

/// Holds all runtime configuration loaded from environment variables.
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    /// Loads configuration from environment variables.
    ///
    /// # Panics
    /// Panics if a required variable is missing or malformed.
    pub fn from_env() -> Self {
        Self {
            database_url: required_var("DATABASE_URL"),
            jwt_secret: required_var("JWT_SECRET"),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
        }
    }

    pub fn socket_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn required_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Missing required environment variable: {name}"))
}
