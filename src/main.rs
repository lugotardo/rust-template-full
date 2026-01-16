use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Aplicação Rust modelo criada com Nix
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Nome do usuário
    #[arg(short, long)]
    name: Option<String>,

    /// Arquivo de configuração JSON
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Modo verbose
    #[arg(short, long)]
    verbose: bool,

    /// Comando a executar
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Saúda o usuário
    Greet {
        /// Nome da pessoa
        name: String,
    },
    /// Processa um arquivo JSON
    Process {
        /// Caminho do arquivo
        file: PathBuf,
    },
    /// Calcula fibonacci
    Fibonacci {
        /// Número para calcular
        n: u64,
    },
    #[cfg(feature = "postgres")]
    /// Comandos de banco de dados
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
}

#[cfg(feature = "postgres")]
#[derive(Parser, Debug)]
enum DbCommands {
    /// Inicializa o banco de dados e executa migrations
    Init,
    /// Testa a conexão com o banco
    Ping,
    /// Cria um novo usuário
    CreateUser {
        /// Nome do usuário
        name: String,
        /// Email do usuário
        email: String,
    },
    /// Lista todos os usuários
    ListUsers,
    /// Busca usuário por ID
    GetUser {
        /// ID do usuário
        id: i32,
    },
    /// Deleta um usuário
    DeleteUser {
        /// ID do usuário
        id: i32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    app_name: String,
    version: String,
    features: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: "rust-app-exemplo".to_string(),
            version: "0.1.0".to_string(),
            features: vec!["cli".to_string(), "json".to_string()],
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🦀 Modo verbose ativado");
        println!("Args: {:?}", args);
    }

    // Carregar configuração se fornecida
    let config = if let Some(config_path) = args.config {
        let content = fs::read_to_string(&config_path)?;
        serde_json::from_str(&content)?
    } else {
        Config::default()
    };

    if args.verbose {
        println!("Configuração: {:?}", config);
    }

    // Executar comando
    match args.command {
        Some(Commands::Greet { name }) => {
            greet(&name);
        }
        Some(Commands::Process { file }) => {
            process_file(file)?;
        }
        Some(Commands::Fibonacci { n }) => {
            let result = fibonacci(n);
            println!("Fibonacci({}) = {}", n, result);
        }
        #[cfg(feature = "postgres")]
        Some(Commands::Db { command }) => {
            handle_db_command(command).await?;
        }
        None => {
            if let Some(name) = args.name {
                greet(&name);
            } else {
                println!("👋 Bem-vindo ao {}!", config.app_name);
                println!("Use --help para ver os comandos disponíveis");
            }
        }
    }

    #[cfg(feature = "postgres")]
    async fn handle_db_command(command: DbCommands) -> Result<()> {
        use rust_app_exemplo::db::{Database, DbUser};

        match command {
            DbCommands::Init => {
                println!("🔧 Inicializando banco de dados...");
                let db = Database::from_env().await?;
                db.migrate().await?;
                println!("✅ Banco de dados inicializado com sucesso!");
                println!("📊 Migrations executadas!");
            }
            DbCommands::Ping => {
                println!("🔍 Testando conexão com o banco...");
                let db = Database::from_env().await?;
                db.ping().await?;
                println!("✅ Conexão OK!");
            }
            DbCommands::CreateUser { name, email } => {
                println!("👤 Criando usuário...");
                let db = Database::from_env().await?;
                let user = DbUser::create(db.pool(), &name, &email).await?;
                println!("✅ Usuário criado com sucesso!");
                println!("{}", serde_json::to_string_pretty(&user)?);
            }
            DbCommands::ListUsers => {
                println!("📋 Listando usuários...");
                let db = Database::from_env().await?;
                let users = DbUser::list_all(db.pool()).await?;
                let count = DbUser::count(db.pool()).await?;

                println!("\n{} usuário(s) encontrado(s):\n", count);
                for user in users {
                    println!(
                        "  [{}] {} - {} ({})",
                        user.id,
                        user.name,
                        user.email,
                        if user.active { "ativo" } else { "inativo" }
                    );
                }
            }
            DbCommands::GetUser { id } => {
                println!("🔍 Buscando usuário #{}...", id);
                let db = Database::from_env().await?;
                match DbUser::find_by_id(db.pool(), id).await? {
                    Some(user) => {
                        println!("✅ Usuário encontrado!");
                        println!("{}", serde_json::to_string_pretty(&user)?);
                    }
                    None => {
                        println!("❌ Usuário não encontrado!");
                    }
                }
            }
            DbCommands::DeleteUser { id } => {
                println!("🗑️  Deletando usuário #{}...", id);
                let db = Database::from_env().await?;
                DbUser::delete(db.pool(), id).await?;
                println!("✅ Usuário deletado com sucesso!");
            }
        }

        Ok(())
    }

    Ok(())
}

fn greet(name: &str) {
    println!("Olá, {}! 👋", name);
    println!("Bem-vindo à aplicação Rust com Nix!");
}

fn process_file(path: PathBuf) -> Result<()> {
    println!("📄 Processando arquivo: {:?}", path);

    let content = fs::read_to_string(&path)?;
    let data: serde_json::Value = serde_json::from_str(&content)?;

    println!("✅ Arquivo processado com sucesso!");
    println!("Conteúdo: {}", serde_json::to_string_pretty(&data)?);

    Ok(())
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.app_name, "rust-app-exemplo");
        assert_eq!(config.version, "0.1.0");
        assert!(config.features.contains(&"cli".to_string()));
    }
}
