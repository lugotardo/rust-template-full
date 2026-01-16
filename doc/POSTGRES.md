# 🐘 PostgreSQL com Nix - Guia Completo

Este guia explica como usar PostgreSQL neste projeto Rust com Nix.

## 🚀 Início Rápido

### 1. Entrar no Shell com PostgreSQL

```bash
nix --extra-experimental-features 'nix-command flakes' develop .#with-postgres
```

Ou use o Makefile:

```bash
make dev-postgres
```

### 2. Iniciar o PostgreSQL

Dentro do shell Nix, execute:

```bash
pg_start
```

Isso irá:
- ✅ Criar o diretório de dados (se não existir)
- ✅ Inicializar o cluster PostgreSQL
- ✅ Iniciar o servidor
- ✅ Criar o banco de dados

### 3. Executar Migrations

```bash
# Compilar com feature postgres
cargo build --features postgres

# Executar migrations
cargo run --features postgres -- db init
```

### 4. Testar a Conexão

```bash
cargo run --features postgres -- db ping
```

## 📋 Comandos PostgreSQL Disponíveis

Dentro do shell `with-postgres`, você tem acesso a estas funções:

| Comando | Descrição |
|---------|-----------|
| `pg_start` | Inicia o PostgreSQL |
| `pg_stop` | Para o PostgreSQL |
| `pg_restart` | Reinicia o PostgreSQL |
| `pg_status` | Mostra o status do servidor |
| `pg_psql` | Conecta ao banco via psql |
| `pg_reset` | **CUIDADO!** Apaga tudo e reinicia |

## 🔧 Configuração

### Variáveis de Ambiente

Ao entrar no shell `with-postgres`, estas variáveis são automaticamente configuradas:

```bash
PGDATA=./.pgdata          # Diretório de dados
PGPORT=5432               # Porta do servidor
PGDATABASE=rust_app_db    # Nome do banco
PGUSER=rust_app_user      # Usuário
PGHOST=localhost          # Host
```

### Personalizar Configuração

Edite o `flake.nix` para mudar os valores padrão:

```nix
# No flake.nix, procure por:
pgVersion = pkgs.postgresql_16;    # Versão do PostgreSQL
pgDataDir = "./.pgdata";           # Diretório de dados
pgPort = "5432";                   # Porta
pgDatabase = "rust_app_db";        # Nome do banco
pgUser = "rust_app_user";          # Usuário
```

### String de Conexão

```
postgresql://rust_app_user@localhost:5432/rust_app_db
```

## 🎯 Comandos CLI da Aplicação

### Banco de Dados

```bash
# Inicializar e executar migrations
cargo run --features postgres -- db init

# Testar conexão
cargo run --features postgres -- db ping

# Criar usuário
cargo run --features postgres -- db create-user "Alice" "alice@example.com"

# Listar usuários
cargo run --features postgres -- db list-users

# Buscar usuário por ID
cargo run --features postgres -- db get-user 1

# Deletar usuário
cargo run --features postgres -- db delete-user 1
```

## 📊 Migrations

### Estrutura

As migrations ficam em `migrations/`:

```
migrations/
├── 20240101000000_create_users_table.sql
└── 20240101000001_seed_users.sql
```

### Criar Nova Migration

1. Crie um arquivo em `migrations/` com timestamp:

```bash
# Formato: YYYYMMDDHHMMSS_description.sql
touch migrations/20240102120000_add_posts_table.sql
```

2. Escreva o SQL:

```sql
-- migrations/20240102120000_add_posts_table.sql
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    content TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

3. Execute as migrations:

```bash
cargo run --features postgres -- db init
```

### Migration Atual

#### 20240101000000_create_users_table.sql

Cria a tabela `users`:

```sql
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

#### 20240101000001_seed_users.sql

Insere dados de exemplo:

```sql
INSERT INTO users (name, email, active) VALUES 
    ('Alice Silva', 'alice@example.com', true),
    ('Bob Santos', 'bob@example.com', true),
    ('Charlie Costa', 'charlie@example.com', false);
```

## 💻 Usando no Código

### Conectar ao Banco

```rust
use rust_app_exemplo::db::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Usando variáveis de ambiente
    let db = Database::from_env().await?;
    
    // Ou com configuração customizada
    use rust_app_exemplo::db::DatabaseConfig;
    
    let config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        database: "rust_app_db".to_string(),
        username: "rust_app_user".to_string(),
        password: None,
        max_connections: 5,
    };
    
    let db = Database::new(config).await?;
    
    Ok(())
}
```

### CRUD de Usuários

```rust
use rust_app_exemplo::db::{Database, DbUser};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Database::from_env().await?;
    let pool = db.pool();
    
    // Criar usuário
    let user = DbUser::create(pool, "João", "joao@example.com").await?;
    println!("Usuário criado: {:?}", user);
    
    // Buscar por ID
    if let Some(user) = DbUser::find_by_id(pool, 1).await? {
        println!("Encontrado: {}", user.name);
    }
    
    // Buscar por email
    if let Some(user) = DbUser::find_by_email(pool, "joao@example.com").await? {
        println!("Email: {}", user.email);
    }
    
    // Listar todos
    let users = DbUser::list_all(pool).await?;
    for user in users {
        println!("- {}: {}", user.id, user.name);
    }
    
    // Contar usuários
    let count = DbUser::count(pool).await?;
    println!("Total: {} usuários", count);
    
    // Deletar usuário
    DbUser::delete(pool, user.id).await?;
    
    Ok(())
}
```

### Queries Customizadas

```rust
use sqlx::Row;

async fn find_active_users(pool: &sqlx::PgPool) -> anyhow::Result<Vec<DbUser>> {
    let users = sqlx::query_as::<_, DbUser>(
        "SELECT * FROM users WHERE active = true ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(users)
}

async fn count_by_domain(pool: &sqlx::PgPool, domain: &str) -> anyhow::Result<i64> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE email LIKE $1"
    )
    .bind(format!("%@{}", domain))
    .fetch_one(pool)
    .await?;
    
    Ok(count)
}
```

## 🔍 Acessando o Banco Diretamente

### Via psql

```bash
# Dentro do shell with-postgres
pg_psql

# Ou manualmente
psql -h localhost -p 5432 -U rust_app_user -d rust_app_db
```

### Comandos psql Úteis

```sql
-- Listar tabelas
\dt

-- Descrever tabela
\d users

-- Ver todos os usuários
SELECT * FROM users;

-- Usuários ativos
SELECT * FROM users WHERE active = true;

-- Contar por status
SELECT active, COUNT(*) FROM users GROUP BY active;

-- Sair
\q
```

## 🧪 Testes com Banco de Dados

### Test Database

Para testes, crie um banco separado:

```bash
# Dentro do psql
CREATE DATABASE rust_app_test;
```

### Exemplo de Teste

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    async fn setup_test_db() -> anyhow::Result<Database> {
        let config = DatabaseConfig {
            database: "rust_app_test".to_string(),
            ..DatabaseConfig::default()
        };
        
        let db = Database::new(config).await?;
        db.migrate().await?;
        
        Ok(db)
    }
    
    #[tokio::test]
    async fn test_create_user() {
        let db = setup_test_db().await.unwrap();
        let pool = db.pool();
        
        let user = DbUser::create(pool, "Test", "test@example.com")
            .await
            .unwrap();
        
        assert_eq!(user.name, "Test");
        assert!(user.active);
    }
}
```

## 🛠️ Troubleshooting

### PostgreSQL não inicia

```bash
# Verificar se já está rodando
pg_status

# Ver logs
cat .pgdata/logfile

# Resetar tudo (CUIDADO!)
pg_reset
```

### Erro de conexão

```bash
# Verificar variáveis de ambiente
echo $PGHOST
echo $PGPORT
echo $PGDATABASE
echo $PGUSER

# Testar conexão manual
psql -h localhost -p 5432 -U rust_app_user -d rust_app_db
```

### Migration falhou

```bash
# Ver versão atual
psql -h localhost -p 5432 -U rust_app_user -d rust_app_db -c "SELECT * FROM _sqlx_migrations;"

# Reverter manualmente se necessário
# (SQLx não tem rollback automático, você precisa criar migrations reversas)
```

### Porta já em uso

Edite o `flake.nix` e mude a porta:

```nix
pgPort = "5433";  # Ou outra porta disponível
```

Depois:

```bash
exit  # Sair do shell
nix develop .#with-postgres  # Entrar novamente
pg_start
```

## 📚 Recursos Adicionais

### SQLx

- [Documentação SQLx](https://docs.rs/sqlx/)
- [SQLx GitHub](https://github.com/launchbadge/sqlx)
- [Compile-time Queries](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

### PostgreSQL

- [Documentação PostgreSQL](https://www.postgresql.org/docs/)
- [Tutorial PostgreSQL](https://www.postgresqltutorial.com/)

### Nix

- [PostgreSQL no Nix](https://nixos.wiki/wiki/PostgreSQL)

## 💡 Dicas

### 1. Auto-start PostgreSQL

Descomente a última linha do `shellHook` em `flake.nix`:

```nix
# Auto-start opcional (descomente se quiser iniciar automaticamente)
pg_start
```

### 2. Backup

```bash
# Backup do banco
pg_dump -h localhost -p 5432 -U rust_app_user rust_app_db > backup.sql

# Restaurar
psql -h localhost -p 5432 -U rust_app_user rust_app_db < backup.sql
```

### 3. Performance

```bash
# Ver queries lentas (dentro do psql)
SELECT * FROM pg_stat_statements ORDER BY total_time DESC LIMIT 10;

# Analisar query
EXPLAIN ANALYZE SELECT * FROM users WHERE email = 'test@example.com';
```

### 4. Conexão Persistente

O SQLx usa connection pooling automaticamente. Configure em `DatabaseConfig`:

```rust
let config = DatabaseConfig {
    max_connections: 10,  // Aumentar para mais concorrência
    ..Default::default()
};
```

---

**🎉 Agora você está pronto para usar PostgreSQL com Rust e Nix!**