use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub ip_address: String,
    pub jwt_secret: String,
    pub port: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().expect("Failed to load .env variables");
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            ip_address: env::var("IP_ADDRESS").expect("Ip address not set"),
            port: env::var("PORT").unwrap_or("8080".to_string()),
        }
    }
}
