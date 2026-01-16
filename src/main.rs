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

fn main() -> Result<()> {
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
        None => {
            if let Some(name) = args.name {
                greet(&name);
            } else {
                println!("👋 Bem-vindo ao {}!", config.app_name);
                println!("Use --help para ver os comandos disponíveis");
            }
        }
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
