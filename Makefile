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

dev-postgres: ## Entra no ambiente de desenvolvimento Nix com PostgreSQL
	$(NIX) develop .#with-postgres

build: ## Compila o projeto em modo debug
	$(CARGO) build

build-postgres: ## Compila o projeto com suporte a PostgreSQL
	$(CARGO) build --features postgres

build-release: ## Compila o projeto em modo release
	$(CARGO) build --release

build-release-postgres: ## Compila com PostgreSQL em modo release
	$(CARGO) build --release --features postgres

nix-build: ## Compila usando Nix
	$(NIX) build

run: ## Executa a aplicação
	$(CARGO) run

run-postgres: ## Executa a aplicação com PostgreSQL
	$(CARGO) run --features postgres

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

# Comandos PostgreSQL
pg-init: ## Inicializa o banco de dados e executa migrations
	$(CARGO) run --features postgres -- db init

pg-ping: ## Testa a conexão com o banco
	$(CARGO) run --features postgres -- db ping

pg-list: ## Lista todos os usuários do banco
	$(CARGO) run --features postgres -- db list-users

pg-create: ## Cria um usuário (uso: make pg-create NAME="João" EMAIL="joao@example.com")
	$(CARGO) run --features postgres -- db create-user "$(NAME)" "$(EMAIL)"

pg-get: ## Busca usuário por ID (uso: make pg-get ID=1)
	$(CARGO) run --features postgres -- db get-user $(ID)

pg-delete: ## Deleta usuário por ID (uso: make pg-delete ID=1)
	$(CARGO) run --features postgres -- db delete-user $(ID)

# Comandos de Deploy
deploy-check: ## Verifica pré-requisitos para deploy
	./deploy/deploy.sh check

deploy-docker: ## Deploy usando Docker Compose
	./deploy/deploy.sh docker

deploy-docker-build: ## Deploy usando Docker Compose com rebuild
	./deploy/deploy.sh docker --build

deploy-docker-tools: ## Deploy Docker com ferramentas (pgAdmin)
	./deploy/deploy.sh docker --with-tools

deploy-systemd: ## Deploy usando systemd (requer root)
	./deploy/deploy.sh systemd

deploy-nix: ## Deploy usando Nix (build local)
	./deploy/deploy.sh nix

deploy-nix-remote: ## Deploy Nix remoto (uso: make deploy-nix-remote TARGET=user@server)
	./deploy/deploy.sh nix --target $(TARGET)

deploy-local: ## Deploy local (compilar e instalar)
	./deploy/deploy.sh local

deploy-cleanup: ## Limpar recursos de deploy
	./deploy/deploy.sh cleanup

# Docker Compose
docker-up: ## Inicia serviços Docker Compose
	docker compose up -d

docker-down: ## Para serviços Docker Compose
	docker compose down

docker-logs: ## Ver logs Docker Compose
	docker compose logs -f

docker-ps: ## Lista containers Docker Compose
	docker compose ps

docker-build: ## Build imagem Docker
	docker build -t rust-app:latest .

docker-exec: ## Executar shell no container (uso: make docker-exec)
	docker compose exec app sh
