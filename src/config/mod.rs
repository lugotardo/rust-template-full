//! Sistema de configuração centralizado
//!
//! Carrega configurações de múltiplas fontes com precedência:
//! 1. Valores padrão
//! 2. Arquivo config.toml
//! 3. Variáveis de ambiente
//! 4. Argumentos CLI

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: Option<String>,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
    pub file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub api_enabled: bool,
    pub metrics_enabled: bool,
    pub cors_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            logging: LoggingConfig::default(),
            features: FeaturesConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: None,
            timeout_seconds: 30,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("PGHOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("PGPORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5432),
            database: std::env::var("PGDATABASE").unwrap_or_else(|_| "rust_app_db".to_string()),
            username: std::env::var("PGUSER").unwrap_or_else(|_| "rust_app_user".to_string()),
            password: std::env::var("PGPASSWORD").ok(),
            max_connections: 10,
            min_connections: 2,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            format: LogFormat::Pretty,
            file: None,
        }
    }
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            api_enabled: true,
            metrics_enabled: false,
            cors_enabled: true,
        }
    }
}

impl AppConfig {
    /// Carrega configuração de múltiplas fontes
    pub fn load() -> anyhow::Result<Self> {
        // Carregar .env se existir
        dotenvy::dotenv().ok();

        let settings = config::Config::builder()
            // Valores padrão
            .add_source(config::Config::try_from(&AppConfig::default())?)
            // Arquivo de configuração (opcional)
            .add_source(config::File::with_name("config").required(false))
            // Variáveis de ambiente com prefixo APP_
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true)
            )
            .build()?;

        let config: AppConfig = settings.try_deserialize()?;
        
        Ok(config)
    }

    /// Retorna a string de conexão do banco de dados
    pub fn database_url(&self) -> String {
        let password = self.database.password
            .as_ref()
            .map(|p| format!(":{}", p))
            .unwrap_or_default();

        format!(
            "postgres://{}{}@{}:{}/{}",
            self.database.username,
            password,
            self.database.host,
            self.database.port,
            self.database.database
        )
    }

    /// Retorna o endereço do servidor
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_database_url() {
        let config = AppConfig {
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                database: "testdb".to_string(),
                username: "testuser".to_string(),
                password: Some("testpass".to_string()),
                max_connections: 5,
                min_connections: 1,
            },
            ..Default::default()
        };

        assert_eq!(
            config.database_url(),
            "postgres://testuser:testpass@localhost:5432/testdb"
        );
    }

    #[test]
    fn test_server_address() {
        let config = AppConfig::default();
        assert_eq!(config.server_address(), "0.0.0.0:8080");
    }
}
