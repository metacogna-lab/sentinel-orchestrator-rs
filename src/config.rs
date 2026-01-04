//! Configuration management for Sentinel Orchestrator
//! Handles environment-specific configuration loading

use anyhow::{Context, Result};
use secrecy::{ExposeSecret, Secret};
use std::path::PathBuf;

/// Application environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    /// Get environment from ENVIRONMENT env var or default to development
    pub fn from_env() -> Self {
        match std::env::var("ENVIRONMENT")
            .as_deref()
            .map(str::to_lowercase)
            .as_deref()
        {
            Ok("production") | Ok("prod") => Self::Production,
            _ => Self::Development,
        }
    }

    /// Get the environment file name
    pub fn env_file(&self) -> &'static str {
        match self {
            Self::Development => ".env.development",
            Self::Production => ".env.production",
        }
    }

    /// Check if this is production
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }

    /// Check if this is development
    pub fn is_development(&self) -> bool {
        matches!(self, Self::Development)
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Production => write!(f, "production"),
        }
    }
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Current environment
    pub environment: Environment,
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// OpenAI API key
    pub openai_api_key: Secret<String>,
    /// Qdrant URL
    pub qdrant_url: String,
    /// Qdrant API key (optional)
    pub qdrant_api_key: Option<Secret<String>>,
    /// Sled storage path
    pub sled_path: PathBuf,
    /// Rust log level
    pub rust_log: String,
    /// Rust backtrace setting
    pub rust_backtrace: String,
    /// Metrics enabled
    pub metrics_enabled: bool,
    /// Metrics port
    pub metrics_port: u16,
    /// CORS allowed origin
    pub cors_allow_origin: String,
    /// Enable debug routes
    pub enable_debug_routes: bool,
    /// Enable metrics export
    pub enable_metrics_export: bool,
}

impl Config {
    /// Load configuration from environment
    ///
    /// This function:
    /// 1. Determines the environment from ENVIRONMENT env var (defaults to development)
    /// 2. Loads the appropriate .env file (.env.development or .env.production)
    /// 3. Loads .env.local if it exists (for local overrides)
    /// 4. Parses all configuration values
    pub fn load() -> Result<Self> {
        // Determine environment
        let environment = Environment::from_env();
        let env_file = environment.env_file();

        // Load environment-specific file first
        if let Err(e) = dotenvy::from_filename(env_file) {
            tracing::warn!(
                "Failed to load {}: {}. Using environment variables only.",
                env_file,
                e
            );
        } else {
            tracing::info!("Loaded environment file: {}", env_file);
        }

        // Load .env.local for local overrides (if exists)
        dotenvy::from_filename(".env.local").ok();
        dotenvy::from_filename(".env.development.local").ok();
        dotenvy::from_filename(".env.production.local").ok();

        // Load standard .env as fallback (for backward compatibility)
        dotenvy::dotenv().ok();

        // Parse configuration
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .context("Invalid PORT value")?;

        let openai_api_key =
            Secret::new(std::env::var("OPENAI_API_KEY").context("OPENAI_API_KEY not set")?);

        let qdrant_url =
            std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());

        let qdrant_api_key = std::env::var("QDRANT_API_KEY")
            .ok()
            .filter(|s| !s.is_empty())
            .map(Secret::new);

        let sled_path = std::env::var("SLED_PATH")
            .unwrap_or_else(|_| "./data/sled".to_string())
            .into();

        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        let rust_backtrace = std::env::var("RUST_BACKTRACE").unwrap_or_else(|_| "0".to_string());

        let metrics_enabled = std::env::var("METRICS_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        let metrics_port = std::env::var("METRICS_PORT")
            .unwrap_or_else(|_| "9090".to_string())
            .parse::<u16>()
            .unwrap_or(9090);

        let cors_allow_origin =
            std::env::var("CORS_ALLOW_ORIGIN").unwrap_or_else(|_| "*".to_string());

        let enable_debug_routes = std::env::var("ENABLE_DEBUG_ROUTES")
            .unwrap_or_else(|_| {
                if environment.is_development() {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            })
            .parse::<bool>()
            .unwrap_or(false);

        let enable_metrics_export = std::env::var("ENABLE_METRICS_EXPORT")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Ok(Self {
            environment,
            host,
            port,
            openai_api_key: Secret::new(openai_api_key.expose_secret().clone()),
            qdrant_url,
            qdrant_api_key,
            sled_path,
            rust_log,
            rust_backtrace,
            metrics_enabled,
            metrics_port,
            cors_allow_origin,
            enable_debug_routes,
            enable_metrics_export,
        })
    }

    /// Get the server address
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_environment_from_env() {
        // Test development (default)
        env::remove_var("ENVIRONMENT");
        assert_eq!(Environment::from_env(), Environment::Development);

        // Test production
        env::set_var("ENVIRONMENT", "production");
        assert_eq!(Environment::from_env(), Environment::Production);

        // Test case insensitive
        env::set_var("ENVIRONMENT", "PRODUCTION");
        assert_eq!(Environment::from_env(), Environment::Production);

        // Test prod shorthand
        env::set_var("ENVIRONMENT", "prod");
        assert_eq!(Environment::from_env(), Environment::Production);

        // Cleanup
        env::remove_var("ENVIRONMENT");
    }

    #[test]
    fn test_environment_file_names() {
        assert_eq!(Environment::Development.env_file(), ".env.development");
        assert_eq!(Environment::Production.env_file(), ".env.production");
    }

    #[test]
    fn test_environment_checks() {
        assert!(Environment::Development.is_development());
        assert!(!Environment::Development.is_production());

        assert!(Environment::Production.is_production());
        assert!(!Environment::Production.is_development());
    }

    #[test]
    fn test_config_load_with_env_vars() {
        // Set required environment variables
        env::set_var("ENVIRONMENT", "development");
        env::set_var("OPENAI_API_KEY", "test-key-123");

        // Create temp directory for sled
        let temp_dir = TempDir::new().unwrap();
        let sled_path = temp_dir.path().join("sled");
        env::set_var("SLED_PATH", sled_path.to_str().unwrap());

        let config = Config::load();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.environment, Environment::Development);
        assert_eq!(config.port, 3000); // Default

        // Cleanup
        env::remove_var("ENVIRONMENT");
        env::remove_var("OPENAI_API_KEY");
        env::remove_var("SLED_PATH");
    }

    #[test]
    fn test_config_server_addr() {
        let config = Config {
            environment: Environment::Development,
            host: "127.0.0.1".to_string(),
            port: 8080,
            openai_api_key: Secret::new("test".to_string()),
            qdrant_url: "http://localhost:6333".to_string(),
            qdrant_api_key: None,
            sled_path: "./data".into(),
            rust_log: "debug".to_string(),
            rust_backtrace: "1".to_string(),
            metrics_enabled: true,
            metrics_port: 9090,
            cors_allow_origin: "*".to_string(),
            enable_debug_routes: true,
            enable_metrics_export: true,
        };

        assert_eq!(config.server_addr(), "127.0.0.1:8080");
    }
}
