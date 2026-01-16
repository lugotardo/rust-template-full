.PHONY: help build run test bench clean fmt lint dev install check all

# Variáveis
CARGO := cargo
NIX := nix --extra-experimental-features 'nix-command flakes'

help: ## Mostra esta mensagem de ajuda
	@echo "🦀 Rust App - Comandos disponíveis:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

dev: ## Entra no ambiente de desenvolvimento Nix
	$(NIX) develop

build: ## Compila o projeto em modo debug
	$(CARGO) build

build-release: ## Compila o projeto em modo release
	$(CARGO) build --release

nix-build: ## Compila usando Nix
	$(NIX) build

run: ## Executa a aplicação
	$(CARGO) run

run-release: ## Executa a aplicação em modo release
	$(CARGO) run --release

test: ## Executa todos os testes
	$(CARGO) test

test-verbose: ## Executa testes com saída verbose
	$(CARGO) test -- --nocapture --test-threads=1

test-integration: ## Executa apenas testes de integração
	$(CARGO) test --test integration_test

bench: ## Executa benchmarks
	$(CARGO) bench

fmt: ## Formata o código
	$(CARGO) fmt

fmt-check: ## Verifica formatação sem modificar
	$(CARGO) fmt -- --check

lint: ## Executa clippy para verificar o código
	$(CARGO) clippy -- -D warnings

lint-fix: ## Tenta corrigir problemas automaticamente
	$(CARGO) clippy --fix

check: ## Verifica se o projeto compila
	$(CARGO) check

doc: ## Gera documentação
	$(CARGO) doc --no-deps --open

clean: ## Limpa artefatos de build
	$(CARGO) clean
	rm -rf result result-*

watch: ## Recompila automaticamente quando arquivos mudam
	$(CARGO) watch -x run

watch-test: ## Executa testes automaticamente quando arquivos mudam
	$(CARGO) watch -x test

install: ## Instala a aplicação localmente
	$(CARGO) install --path .

update: ## Atualiza dependências
	$(CARGO) update

lockfile: ## Gera Cargo.lock
	$(CARGO) generate-lockfile

all: fmt lint test build ## Executa formatação, lint, testes e build

ci: fmt-check lint test build-release ## Pipeline de CI completo

# Exemplos de execução
example-greet: ## Exemplo: saudar
	$(CARGO) run -- greet "Mundo"

example-fibonacci: ## Exemplo: calcular fibonacci
	$(CARGO) run -- fibonacci 10

example-verbose: ## Exemplo: modo verbose
	$(CARGO) run -- --verbose greet "Nix"

# Inicialização
init: ## Inicializa o projeto (gera Cargo.lock)
	git init
	git add .
	$(CARGO) generate-lockfile
	@echo "✅ Projeto inicializado!"

# Utilitários
tree: ## Mostra árvore de dependências
	$(CARGO) tree

outdated: ## Verifica dependências desatualizadas
	$(CARGO) outdated

audit: ## Verifica vulnerabilidades de segurança
	$(CARGO) audit
