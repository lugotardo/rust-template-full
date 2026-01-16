# 🏗️ Arquitetura do Projeto

Este documento descreve a arquitetura e organização da aplicação Rust modelo com Nix.

## 📁 Estrutura de Diretórios

```
nixtest/
├── flake.nix                 # Configuração Nix Flakes
├── flake.lock                # Lock file do Nix (gerado)
├── Cargo.toml                # Manifesto do projeto Rust
├── Cargo.lock                # Lock file de dependências Rust
├── Makefile                  # Comandos úteis automatizados
├── setup.sh                  # Script de configuração inicial
├── .gitignore                # Arquivos ignorados pelo Git
├── README.md                 # Documentação principal
├── QUICKSTART.md             # Guia de início rápido
├── ARCHITECTURE.md           # Este arquivo
├── config.example.json       # Exemplo de configuração
│
├── src/
│   ├── main.rs              # Ponto de entrada da aplicação CLI
│   └── lib.rs               # Biblioteca com funções reutilizáveis
│
├── tests/
│   └── integration_test.rs  # Testes de integração
│
└── benches/
    └── benchmarks.rs        # Benchmarks de performance
```

## 🔧 Componentes Principais

### 1. Nix Flakes (`flake.nix`)

**Responsabilidades:**
- Gerenciar dependências do sistema
- Configurar toolchain Rust
- Definir ambiente de desenvolvimento
- Configurar build reproduzível

**Inputs:**
- `nixpkgs`: Repositório de pacotes Nix
- `rust-overlay`: Overlay para versões específicas do Rust
- `flake-utils`: Utilitários para multi-plataforma

**Outputs:**
- `packages.default`: Build do binário final
- `devShells.default`: Shell de desenvolvimento

### 2. Aplicação CLI (`src/main.rs`)

**Arquitetura:**
```
Args (clap)
    ↓
Commands (enum)
    ↓
    ├── Greet
    ├── Process
    └── Fibonacci
    ↓
Funções de Execução
    ↓
Result<()>
```

**Componentes:**

#### Args Struct
- Parser de argumentos CLI usando `clap`
- Suporta flags e subcomandos
- Validação automática de entrada

#### Commands Enum
- `Greet`: Saúda um usuário
- `Process`: Processa arquivos JSON
- `Fibonacci`: Calcula números de Fibonacci

#### Config Struct
- Configuração serializável em JSON
- Valores padrão implementados
- Suporte a arquivo externo

### 3. Biblioteca (`src/lib.rs`)

**Módulos:**

#### User Management
```rust
pub struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}
```

**Features:**
- Criação e gerenciamento de usuários
- Serialização/deserialização JSON
- Ativação/desativação de contas

#### Funções Matemáticas
- `fibonacci_optimized(n)`: Fibonacci iterativo O(n)
- `factorial(n)`: Fatorial usando product
- `is_prime(n)`: Teste de primalidade otimizado

#### String Utilities
- `to_title_case()`: Conversão para title case
- `count_vowels()`: Contagem de vogais
- `reverse()`: Inversão de strings

## 🔄 Fluxo de Dados

### Inicialização
```
main()
    ↓
Args::parse()
    ↓
Config::load()
    ↓
Command::execute()
    ↓
Result
```

### Processamento de JSON
```
File Path
    ↓
fs::read_to_string()
    ↓
serde_json::from_str()
    ↓
Value
    ↓
serde_json::to_string_pretty()
    ↓
Output
```

## 🧪 Estratégia de Testes

### Testes Unitários (`src/lib.rs`, `src/main.rs`)
- Testam funções isoladas
- Executados com `#[test]`
- Localizados no mesmo arquivo da implementação

**Exemplo:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_optimized(10), 55);
    }
}
```

### Testes de Integração (`tests/`)
- Testam múltiplos componentes juntos
- Usam a biblioteca como dependência externa
- Validam comportamento end-to-end

**Exemplo:**
```rust
use rust_app_exemplo::*;

#[test]
fn test_user_lifecycle() {
    let mut user = User::new(...);
    user.deactivate();
    assert!(!user.active);
}
```

### Benchmarks (`benches/`)
- Medem performance de funções críticas
- Usam Criterion para resultados estatísticos
- Geram relatórios HTML

## 📦 Dependências

### Produção

| Crate | Versão | Uso |
|-------|--------|-----|
| `clap` | 4.4 | Parser de CLI com derive macros |
| `serde` | 1.0 | Serialização/deserialização |
| `serde_json` | 1.0 | Suporte a JSON |
| `tokio` | 1.35 | Runtime assíncrono |
| `anyhow` | 1.0 | Error handling simplificado |
| `thiserror` | 1.0 | Macros para custom errors |

### Desenvolvimento

| Crate | Versão | Uso |
|-------|--------|-----|
| `criterion` | 0.5 | Framework de benchmarking |

### Ferramentas Nix

- `rustc`: Compilador Rust
- `cargo`: Gerenciador de pacotes
- `rust-analyzer`: LSP para IDEs
- `cargo-watch`: Auto-recompilação
- `cargo-edit`: Gerenciar dependências
- `rustfmt`: Formatação de código
- `clippy`: Linting

## 🔐 Gerenciamento de Erros

### Estratégia

```rust
Result<T, E>
    ↓
anyhow::Result<T>  // Para aplicação
    ↓
thiserror::Error   // Para biblioteca
```

### Propagação de Erros

- Uso de `?` operator para propagação
- Conversão automática com `anyhow`
- Mensagens de erro contextualizadas

## 🚀 Build e Deploy

### Desenvolvimento
```
cargo build
    ↓
target/debug/rust-app-exemplo
```

### Release
```
cargo build --release
    ↓
target/release/rust-app-exemplo
```

### Nix Build (Reproduzível)
```
nix build
    ↓
result/bin/rust-app-exemplo
```

**Vantagens do Nix Build:**
- ✅ Reproduzível em qualquer máquina
- ✅ Hash SHA256 único
- ✅ Cache compartilhado
- ✅ Dependências isoladas

## 🎯 Padrões de Design

### 1. Builder Pattern
Usado em `clap` para construção de parsers:
```rust
#[derive(Parser)]
struct Args { ... }
```

### 2. Error Handling Pattern
Uso consistente de `Result<T>`:
```rust
fn operation() -> Result<T> {
    // operação que pode falhar
    Ok(value)
}
```

### 3. Module Pattern
Organização em módulos públicos e privados:
```rust
pub mod string_utils {
    pub fn to_title_case(...) { }
}
```

### 4. Default Pattern
Implementação de valores padrão:
```rust
impl Default for Config {
    fn default() -> Self { ... }
}
```

## 🔄 Ciclo de Vida do Desenvolvimento

```
┌─────────────────┐
│  Editar Código  │
└────────┬────────┘
         ↓
┌─────────────────┐
│  cargo check    │
└────────┬────────┘
         ↓
┌─────────────────┐
│  cargo test     │
└────────┬────────┘
         ↓
┌─────────────────┐
│  cargo clippy   │
└────────┬────────┘
         ↓
┌─────────────────┐
│  cargo fmt      │
└────────┬────────┘
         ↓
┌─────────────────┐
│  cargo build    │
└────────┬────────┘
         ↓
┌─────────────────┐
│  git commit     │
└─────────────────┘
```

## 🌐 Extensibilidade

### Adicionar Novo Comando

1. Adicione variant ao enum `Commands`:
```rust
enum Commands {
    NovoComando { arg: String },
}
```

2. Implemente o handler:
```rust
Some(Commands::NovoComando { arg }) => {
    executar_novo_comando(arg)?;
}
```

### Adicionar Nova Função à Biblioteca

1. Implemente em `src/lib.rs`:
```rust
pub fn nova_funcao() -> Result<T> {
    // implementação
}
```

2. Adicione testes:
```rust
#[test]
fn test_nova_funcao() {
    assert_eq!(nova_funcao(), expected);
}
```

### Adicionar Dependência

```bash
cargo add nome-da-crate
```

O Nix detectará automaticamente via `Cargo.lock`.

## 📊 Performance

### Otimizações de Release

```toml
[profile.release]
opt-level = 3        # Otimização máxima
lto = true           # Link Time Optimization
codegen-units = 1    # Melhor otimização, build mais lento
```

### Benchmarks

Execute com:
```bash
cargo bench
```

Resultados em `target/criterion/report/index.html`

## 🔍 Debugging

### Ambiente de Desenvolvimento

```bash
# Compilar com debug symbols
cargo build

# Executar com RUST_BACKTRACE
RUST_BACKTRACE=1 cargo run

# Usar GDB/LLDB
rust-gdb target/debug/rust-app-exemplo
```

### Logging

Adicione `env_logger` ou `tracing` para logs estruturados.

## 📝 Convenções de Código

- **Formatação**: `rustfmt` (executar com `cargo fmt`)
- **Linting**: `clippy` (executar com `cargo clippy`)
- **Nomenclatura**: snake_case para funções, PascalCase para tipos
- **Documentação**: Comentários `///` para items públicos

## 🎓 Recursos e Referências

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Nix Manual](https://nixos.org/manual/nix/stable/)
- [Clap Documentation](https://docs.rs/clap/)
- [Serde Guide](https://serde.rs/)

---

**Última atualização:** 2025
**Versão:** 0.1.0