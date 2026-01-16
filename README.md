# 🦀 Aplicação Rust Modelo com Nix

Uma aplicação Rust completa configurada com Nix Flakes para builds reproduzíveis e gerenciamento de dependências.

## 📋 Características

- ✅ Estrutura de projeto Rust moderna
- ✅ Gerenciamento de dependências com Nix Flakes
- ✅ CLI interativa com `clap`
- ✅ Serialização JSON com `serde`
- ✅ **PostgreSQL 16 integrado** (opcional com feature flag)
- ✅ SQLx para queries type-safe
- ✅ Migrations automáticas
- ✅ Testes unitários e de integração
- ✅ Benchmarks com Criterion
- ✅ Ambiente de desenvolvimento configurado

## 🚀 Começando

### Pré-requisitos

- Nix com suporte a Flakes habilitado

Para habilitar Flakes, adicione ao seu `~/.config/nix/nix.conf`:

```
experimental-features = nix-command flakes
```

### Entrando no ambiente de desenvolvimento

```bash
nix develop
```

Isso irá carregar um shell com todas as ferramentas necessárias:
- Rust toolchain (rustc, cargo)
- rust-analyzer
- cargo-watch
- cargo-edit
- rustfmt
- clippy

### Ou com PostgreSQL habilitado

```bash
nix --extra-experimental-features 'nix-command flakes' develop .#with-postgres
```

Isso inclui tudo acima mais:
- PostgreSQL 16
- Funções auxiliares (pg_start, pg_stop, etc.)
- Variáveis de ambiente configuradas

📖 **Veja [POSTGRES.md](POSTGRES.md) para documentação completa do PostgreSQL**

### Compilando o projeto

```bash
# Modo desenvolvimento
cargo build

# Modo release (otimizado)
cargo build --release
```

### Build com Nix

```bash
# Build completo com Nix
nix build

# O executável estará em ./result/bin/rust-app-exemplo
./result/bin/rust-app-exemplo --help
```

## 🎯 Uso

### Comandos disponíveis

```bash
# Ajuda geral
cargo run -- --help

# Saudar alguém
cargo run -- greet "Mundo"

# Calcular Fibonacci
cargo run -- fibonacci 10

# Processar arquivo JSON (crie um arquivo de teste primeiro)
echo '{"name": "test", "value": 42}' > test.json
cargo run -- process test.json

# Modo verbose
cargo run -- --verbose greet "Nix"

# Comandos de banco de dados (requer feature postgres)
cargo run --features postgres -- db init
cargo run --features postgres -- db list-users
cargo run --features postgres -- db create-user "João" "joao@example.com"
```

## 🧪 Testes

### Executar testes unitários

```bash
cargo test
```

### Executar testes com saída detalhada

```bash
cargo test -- --nocapture
```

### Executar testes de integração

```bash
cargo test --test integration_test
```

## 📊 Benchmarks

```bash
cargo bench
```

Os resultados serão salvos em `target/criterion/`.

## 📁 Estrutura do Projeto

```
nixtest/
├── flake.nix              # Configuração Nix Flakes
├── Cargo.toml             # Configuração do projeto Rust
├── Cargo.lock             # Lock file das dependências
├── src/
│   ├── main.rs            # Aplicação CLI principal
│   └── lib.rs             # Biblioteca com utilitários
├── tests/
│   └── integration_test.rs # Testes de integração
├── benches/
│   └── benchmarks.rs      # Benchmarks de performance
└── README.md              # Este arquivo
```

## 🛠️ Desenvolvimento

### Formatação de código

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Watch mode (recompila automaticamente)

```bash
cargo watch -x run
```

## 📦 Funcionalidades da Biblioteca

### User Management

```rust
use rust_app_exemplo::User;

let mut user = User::new(1, "João".to_string(), "joao@example.com".to_string());
user.deactivate();
user.activate();
```

### Funções Matemáticas

```rust
use rust_app_exemplo::{fibonacci_optimized, factorial, is_prime};

let fib = fibonacci_optimized(10);  // 55
let fact = factorial(5);             // 120
let prime = is_prime(7);             // true
```

### Utilitários de String

```rust
use rust_app_exemplo::string_utils;

let title = string_utils::to_title_case("hello world");  // "Hello World"
let vowels = string_utils::count_vowels("hello");         // 2
let reversed = string_utils::reverse("rust");             // "tsur"
```

## 🔧 Personalização

### Adicionar novas dependências

```bash
# No shell de desenvolvimento Nix
cargo add nome-da-dependencia
```

### Modificar a configuração Nix

Edite o arquivo `flake.nix` para:
- Adicionar ferramentas ao ambiente de desenvolvimento
- Modificar a versão do Rust
- Adicionar dependências do sistema

## 📝 Exemplo de Configuração JSON

Crie um arquivo `config.json`:

```json
{
  "app_name": "MinhaApp",
  "version": "1.0.0",
  "features": ["api", "cli", "web"]
}
```

Execute com:

```bash
cargo run -- --config config.json
```

## 🐳 Build Reproduzível

Uma das grandes vantagens de usar Nix é a garantia de builds reproduzíveis. O mesmo código sempre produzirá o mesmo binário, independente da máquina.

```bash
# Verificar hash do build
nix build --print-out-paths
```

## 🤝 Contribuindo

1. Clone o repositório
2. Entre no ambiente Nix: `nix develop`
3. Faça suas modificações
4. Execute os testes: `cargo test`
5. Formate o código: `cargo fmt`
6. Verifique com clippy: `cargo clippy`

## 🐘 PostgreSQL

Este projeto inclui suporte completo a PostgreSQL 16 usando SQLx.

### Início Rápido com Postgres

```bash
# 1. Entrar no shell com PostgreSQL
nix --extra-experimental-features 'nix-command flakes' develop .#with-postgres

# 2. Iniciar o PostgreSQL
pg_start

# 3. Executar migrations
cargo run --features postgres -- db init

# 4. Criar um usuário
cargo run --features postgres -- db create-user "Alice" "alice@example.com"

# 5. Listar usuários
cargo run --features postgres -- db list-users
```

📖 **Documentação completa:** [POSTGRES.md](POSTGRES.md)

### Features do PostgreSQL

- ✅ PostgreSQL 16 gerenciado pelo Nix
- ✅ SQLx com compile-time checked queries
- ✅ Migrations automáticas
- ✅ Connection pooling
- ✅ CRUD completo de exemplo
- ✅ Comandos CLI prontos
- ✅ Funções auxiliares (pg_start, pg_stop, etc.)

## 📄 Licença

MIT

## 🎓 Recursos Adicionais

- [Documentação Rust](https://doc.rust-lang.org/)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)
- [rust-overlay](https://github.com/oxalica/rust-overlay)
- [Clap Documentation](https://docs.rs/clap/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [PostgreSQL Docs](https://www.postgresql.org/docs/)

---

Desenvolvido com ❤️ usando Rust e Nix