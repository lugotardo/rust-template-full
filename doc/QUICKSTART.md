# 🚀 Guia de Início Rápido

## Primeiros Passos em 5 Minutos

### 1. Setup Inicial

Execute o script de setup automatizado:

```bash
./setup.sh
```

Este script irá:
- ✅ Verificar se Nix está instalado
- ✅ Verificar se Flakes está habilitado
- ✅ Inicializar repositório Git
- ✅ Gerar `Cargo.lock`

### 2. Entrar no Ambiente de Desenvolvimento

```bash
nix --extra-experimental-features 'nix-command flakes' develop
```

Ou simplesmente:

```bash
make dev
```

Quando o shell carregar, você verá:

```
🦀 Ambiente Rust com Nix carregado!
Rust version: rustc 1.x.x
Cargo version: cargo 1.x.x
```

### 3. Compilar e Executar

```bash
# Compilar
cargo build

# Executar com ajuda
cargo run -- --help

# Exemplo: Saudar alguém
cargo run -- greet "Mundo"
```

Saída esperada:
```
Olá, Mundo! 👋
Bem-vindo à aplicação Rust com Nix!
```

## 📚 Exemplos Rápidos

### Calcular Fibonacci

```bash
cargo run -- fibonacci 10
```

Saída:
```
Fibonacci(10) = 55
```

### Processar JSON

Crie um arquivo de teste:

```bash
echo '{"nome": "João", "idade": 25}' > teste.json
```

Processe o arquivo:

```bash
cargo run -- process teste.json
```

### Modo Verbose

```bash
cargo run -- --verbose greet "Nix"
```

## 🧪 Executar Testes

```bash
# Todos os testes
cargo test

# Com saída detalhada
cargo test -- --nocapture

# Apenas testes de integração
cargo test --test integration_test

# Apenas testes unitários
cargo test --lib
```

## 📊 Benchmarks

```bash
cargo bench
```

Resultados serão salvos em `target/criterion/report/index.html`

## 🛠️ Comandos Úteis

### Formatação e Linting

```bash
# Formatar código
cargo fmt

# Verificar formatação
cargo fmt -- --check

# Linting com clippy
cargo clippy

# Corrigir problemas automaticamente
cargo clippy --fix
```

### Watch Mode (Auto-recompilação)

```bash
# Recompila quando arquivos mudam
cargo watch -x run

# Executa testes quando arquivos mudam
cargo watch -x test
```

### Documentação

```bash
# Gerar e abrir documentação
cargo doc --open
```

## 🎯 Usando a Biblioteca

### Exemplo 1: User Management

```rust
use rust_app_exemplo::User;

fn main() {
    let mut user = User::new(
        1,
        "Maria Silva".to_string(),
        "maria@example.com".to_string()
    );

    println!("Usuário: {}", user);

    user.deactivate();
    println!("Ativo: {}", user.active); // false

    user.activate();
    println!("Ativo: {}", user.active); // true
}
```

### Exemplo 2: Funções Matemáticas

```rust
use rust_app_exemplo::{fibonacci_optimized, factorial, is_prime};

fn main() {
    println!("Fibonacci(15) = {}", fibonacci_optimized(15));
    println!("5! = {}", factorial(5));
    println!("7 é primo? {}", is_prime(7));
}
```

### Exemplo 3: Utilitários de String

```rust
use rust_app_exemplo::string_utils;

fn main() {
    let texto = "olá mundo";
    println!("Title Case: {}", string_utils::to_title_case(texto));
    println!("Vogais: {}", string_utils::count_vowels(texto));
    println!("Invertido: {}", string_utils::reverse(texto));
}
```

## 🔧 Comandos Make

Use `make help` para ver todos os comandos disponíveis:

```bash
make help
```

Comandos mais úteis:

| Comando | Descrição |
|---------|-----------|
| `make build` | Compila em modo debug |
| `make build-release` | Compila em modo release |
| `make test` | Executa testes |
| `make bench` | Executa benchmarks |
| `make fmt` | Formata código |
| `make lint` | Executa clippy |
| `make clean` | Limpa artefatos |
| `make watch` | Auto-recompila |
| `make all` | fmt + lint + test + build |

## 🐳 Build com Nix

### Build Reproduzível

```bash
nix --extra-experimental-features 'nix-command flakes' build
```

O binário estará em:
```bash
./result/bin/rust-app-exemplo
```

Execute diretamente:
```bash
./result/bin/rust-app-exemplo --help
```

### Executar sem Build

```bash
nix --extra-experimental-features 'nix-command flakes' run
```

## 📦 Adicionar Novas Dependências

Dentro do shell Nix:

```bash
# Adicionar dependência
cargo add serde

# Adicionar dependência de desenvolvimento
cargo add --dev proptest

# Adicionar com features específicas
cargo add tokio --features full

# Remover dependência
cargo rm serde
```

## 🔍 Verificar Status do Projeto

```bash
# Verificar se compila
cargo check

# Árvore de dependências
cargo tree

# Verificar dependências desatualizadas (requer cargo-outdated)
cargo outdated

# Verificar vulnerabilidades (requer cargo-audit)
cargo audit
```

## 💡 Dicas

### 1. Alias para Nix

Adicione ao seu `~/.bashrc` ou `~/.zshrc`:

```bash
alias nix-dev='nix --extra-experimental-features "nix-command flakes" develop'
alias nix-build='nix --extra-experimental-features "nix-command flakes" build'
alias nix-run='nix --extra-experimental-features "nix-command flakes" run'
```

### 2. Editor com rust-analyzer

O ambiente Nix já inclui `rust-analyzer`. Configure seu editor:

**VSCode**: Instale a extensão "rust-analyzer"

**Vim/Neovim**: Configure com CoC ou LSP nativo

### 3. Habilitar Flakes Permanentemente

Edite `~/.config/nix/nix.conf`:

```
experimental-features = nix-command flakes
```

## ❓ Solução de Problemas

### "Git tree is dirty"

```bash
git add .
```

### "Cargo.lock não encontrado"

```bash
cargo generate-lockfile
git add Cargo.lock
```

### Download lento do Nix

Use um cache binário:

```bash
# Em ~/.config/nix/nix.conf
substituters = https://cache.nixos.org https://nix-community.cachix.org
trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs=
```

### Rust não encontrado fora do Nix

O Rust só está disponível dentro do `nix develop`. Entre no shell primeiro.

## 🎓 Próximos Passos

1. ✅ Explore o código em `src/main.rs` e `src/lib.rs`
2. ✅ Modifique e adicione suas próprias funções
3. ✅ Escreva testes para seu código
4. ✅ Execute benchmarks para otimizar
5. ✅ Compile com `nix build` para produção

## 📖 Documentação Completa

Para mais detalhes, veja:
- [README.md](README.md) - Documentação completa
- [Documentação Rust](https://doc.rust-lang.org/)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)

---

**Divirta-se codificando! 🦀**