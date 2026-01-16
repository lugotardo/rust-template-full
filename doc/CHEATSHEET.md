# 🚀 Cheat Sheet - Comandos Rápidos

## 🎯 Setup Inicial

```bash
# Executar setup automatizado
./setup.sh

# Ou manualmente
git init
git add .
cargo generate-lockfile
```

## 🔧 Nix Commands

```bash
# Entrar no shell de desenvolvimento
nix --extra-experimental-features 'nix-command flakes' develop

# Ou com alias curto
make dev

# Build com Nix (reproduzível)
nix --extra-experimental-features 'nix-command flakes' build

# Executar diretamente com Nix
nix --extra-experimental-features 'nix-command flakes' run

# Executar o binário buildado
./result/bin/rust-app-exemplo --help
```

## 🦀 Cargo Commands

### Build & Run

```bash
# Compilar (debug)
cargo build

# Compilar (release/otimizado)
cargo build --release

# Verificar se compila (mais rápido)
cargo check

# Executar
cargo run

# Executar com argumentos
cargo run -- --help
cargo run -- greet "Nome"
cargo run -- fibonacci 10
```

### Testes

```bash
# Todos os testes
cargo test

# Testes com output
cargo test -- --nocapture

# Testes específicos
cargo test test_fibonacci

# Apenas testes unitários
cargo test --lib

# Apenas testes de integração
cargo test --test integration_test

# Com coverage (requer tarpaulin)
cargo tarpaulin
```

### Benchmarks

```bash
# Executar benchmarks
cargo bench

# Benchmark específico
cargo bench fibonacci

# Ver resultados
open target/criterion/report/index.html
```

### Qualidade de Código

```bash
# Formatação
cargo fmt
cargo fmt -- --check  # Apenas verificar

# Linting
cargo clippy
cargo clippy -- -D warnings  # Tratar warnings como erros
cargo clippy --fix  # Auto-fix

# Documentação
cargo doc
cargo doc --open  # Abrir no browser
```

### Dependências

```bash
# Adicionar dependência
cargo add serde

# Adicionar dev dependency
cargo add --dev proptest

# Adicionar com features
cargo add tokio --features full

# Remover dependência
cargo rm serde

# Atualizar dependências
cargo update

# Árvore de dependências
cargo tree

# Verificar desatualizadas (requer cargo-outdated)
cargo outdated

# Audit de segurança (requer cargo-audit)
cargo audit
```

## 🛠️ Make Commands

```bash
# Ver todos os comandos
make help

# Build
make build              # Debug build
make build-release      # Release build
make nix-build          # Build with Nix

# Testes
make test               # Executar testes
make test-verbose       # Testes com output
make test-integration   # Apenas integração

# Qualidade
make fmt                # Formatar código
make fmt-check          # Verificar formatação
make lint               # Clippy
make lint-fix           # Auto-fix com clippy

# Development
make watch              # Auto-recompila
make watch-test         # Auto-testa
make dev                # Shell Nix

# Utilitários
make clean              # Limpar builds
make doc                # Gerar docs
make all                # fmt + lint + test + build
make ci                 # Pipeline completo
```

## 📝 Exemplos de Uso da Aplicação

```bash
# Ajuda geral
cargo run -- --help

# Saudar alguém
cargo run -- greet "Mundo"

# Calcular Fibonacci
cargo run -- fibonacci 15

# Processar JSON
echo '{"test": "value"}' > test.json
cargo run -- process test.json

# Com configuração customizada
cargo run -- --config config.example.json greet "User"

# Modo verbose
cargo run -- --verbose greet "Debug"

# Passar nome via flag
cargo run -- --name "João"
```

## 🔍 Debugging

```bash
# Backtrace completo
RUST_BACKTRACE=1 cargo run

# Backtrace detalhado
RUST_BACKTRACE=full cargo run

# Com logs (se usar env_logger)
RUST_LOG=debug cargo run

# GDB
rust-gdb target/debug/rust-app-exemplo

# LLDB
rust-lldb target/debug/rust-app-exemplo
```

## 📦 Desenvolvimento

```bash
# Watch mode - recompila ao salvar
cargo watch -x run
cargo watch -x test
cargo watch -x check

# Executar formatação antes de commit
cargo fmt && git add -A

# Pipeline completo antes de commit
make ci
```

## 🌐 Git Workflow

```bash
# Commit padrão
git add .
git commit -m "feat: nova funcionalidade"

# Antes de commit importante
make all
git add .
git commit -m "release: versão 1.0.0"

# Ver status
git status
git diff
```

## 🎨 Aliases Úteis (Adicionar ao ~/.bashrc ou ~/.zshrc)

```bash
# Nix
alias nix-dev='nix --extra-experimental-features "nix-command flakes" develop'
alias nix-build='nix --extra-experimental-features "nix-command flakes" build'
alias nix-run='nix --extra-experimental-features "nix-command flakes" run'

# Cargo
alias cb='cargo build'
alias cbr='cargo build --release'
alias cr='cargo run'
alias ct='cargo test'
alias cc='cargo check'
alias cf='cargo fmt'
alias cl='cargo clippy'
alias cw='cargo watch -x run'

# Combinados
alias ccheck='cargo fmt && cargo clippy && cargo test'
alias cbuild='cargo fmt && cargo clippy && cargo test && cargo build --release'
```

## 📊 Performance

```bash
# Build otimizado
cargo build --release

# Executar otimizado
cargo run --release

# Benchmarks
cargo bench

# Profiling (Linux)
cargo build --release
perf record target/release/rust-app-exemplo
perf report

# Flamegraph (requer cargo-flamegraph)
cargo flamegraph
```

## 🔐 Segurança

```bash
# Audit de vulnerabilidades
cargo audit

# Verificar licenças
cargo tree --format "{p} {l}"

# Atualizar dependências com vulnerabilidades
cargo update
```

## 📚 Documentação

```bash
# Gerar docs
cargo doc

# Abrir docs no browser
cargo doc --open

# Docs sem dependências
cargo doc --no-deps

# Docs privados também
cargo doc --document-private-items
```

## 🚨 Troubleshooting

```bash
# Limpar e rebuildar
cargo clean
cargo build

# Atualizar toolchain
rustup update

# Verificar versões
rustc --version
cargo --version
nix --version

# Rebuild do lock file
rm Cargo.lock
cargo generate-lockfile

# Problemas com Nix cache
nix-collect-garbage
nix-store --verify --check-contents
```

## 💡 Dicas Rápidas

```bash
# Compilação mais rápida (desenvolvimento)
cargo build --jobs $(nproc)

# Apenas verificar sintaxe (muito rápido)
cargo check

# Executar teste específico
cargo test nome_do_teste -- --nocapture

# Ver tamanho do binário
ls -lh target/release/rust-app-exemplo

# Reduzir tamanho do binário
strip target/release/rust-app-exemplo

# Comparar performance
hyperfine 'cargo run --release -- fibonacci 20'
```

## 🎯 Workflow Recomendado

### Durante Desenvolvimento
```bash
# Terminal 1: Watch mode
cargo watch -x check -x test

# Terminal 2: Editar código
vim src/main.rs

# Antes de commit
make all
```

### Antes de Release
```bash
# 1. Testes completos
cargo test --release

# 2. Benchmarks
cargo bench

# 3. Clippy pedante
cargo clippy -- -W clippy::pedantic

# 4. Build final
nix build

# 5. Testar binário
./result/bin/rust-app-exemplo --version
```

---

**💡 Dica:** Mantenha este arquivo aberto em um terminal separado para referência rápida!