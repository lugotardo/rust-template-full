//! Módulo de banco de dados PostgreSQL usando SQLx
//!
//! Este módulo só está disponível quando a feature "postgres" está habilitada.

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use serde::{Deserialize, Serialize};

/// Configuração do banco de dados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: Option<String>,
    pub max_connections: u32,
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
            max_connections: 5,
        }
    }
}

impl DatabaseConfig {
    /// Cria uma connection string PostgreSQL
    pub fn connection_string(&self) -> String {
        let password = self
            .password
            .as_ref()
            .map(|p| format!(":{}", p))
            .unwrap_or_default();

        format!(
            "postgres://{}{}@{}:{}/{}",
            self.username, password, self.host, self.port, self.database
        )
    }
}

/// Pool de conexões do banco de dados
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Cria uma nova instância do banco de dados
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.connection_string())
            .await?;

        Ok(Self { pool })
    }

    /// Cria usando variáveis de ambiente
    pub async fn from_env() -> Result<Self> {
        Self::new(DatabaseConfig::default()).await
    }

    /// Retorna uma referência ao pool de conexões
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Verifica se a conexão está funcionando
    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Executa as migrations
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }
}

/// Exemplo de modelo de usuário no banco de dados
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl DbUser {
    /// Cria um novo usuário no banco
    pub async fn create(pool: &PgPool, name: &str, email: &str) -> Result<Self> {
        let user = sqlx::query_as::<_, DbUser>(
            "INSERT INTO users (name, email, active) VALUES ($1, $2, true) RETURNING *"
        )
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Busca um usuário por ID
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, DbUser>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    /// Busca um usuário por email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, DbUser>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    /// Lista todos os usuários
    pub async fn list_all(pool: &PgPool) -> Result<Vec<Self>> {
        let users = sqlx::query_as::<_, DbUser>("SELECT * FROM users ORDER BY id")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    /// Atualiza um usuário
    pub async fn update(&self, pool: &PgPool) -> Result<()> {
        sqlx::query("UPDATE users SET name = $1, email = $2, active = $3 WHERE id = $4")
            .bind(&self.name)
            .bind(&self.email)
            .bind(self.active)
            .bind(self.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Deleta um usuário
    pub async fn delete(pool: &PgPool, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Conta quantos usuários existem
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
    }

    #[test]
    fn test_connection_string() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            database: "testdb".to_string(),
            username: "testuser".to_string(),
            password: Some("testpass".to_string()),
            max_connections: 5,
        };

        let conn_str = config.connection_string();
        assert_eq!(conn_str, "postgres://testuser:testpass@localhost:5432/testdb");
    }

    #[test]
    fn test_connection_string_without_password() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            database: "testdb".to_string(),
            username: "testuser".to_string(),
            password: None,
            max_connections: 5,
        };

        let conn_str = config.connection_string();
        assert_eq!(conn_str, "postgres://testuser@localhost:5432/testdb");
    }
}
