#!/usr/bin/env bash

set -euo pipefail

# ============================================================================
# Script de Deploy - Aplicação Rust com PostgreSQL
# ============================================================================

VERSION="1.0.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Funções de print
print_header() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  $1"
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo ""
}

print_step() {
    echo -e "${CYAN}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_info() {
    echo -e "${MAGENTA}ℹ${NC} $1"
}

# Função de ajuda
show_help() {
    cat << EOF
Deploy Script v${VERSION}

Uso: $0 [COMANDO] [OPÇÕES]

COMANDOS:
    docker          Deploy usando Docker Compose
    systemd         Deploy usando systemd service
    nix             Deploy usando Nix (build reproduzível)
    local           Deploy local (compilar e instalar)
    check           Verificar pré-requisitos
    cleanup         Limpar recursos antigos
    help            Mostrar esta mensagem

OPÇÕES DOCKER:
    --build         Rebuildar imagens
    --no-cache      Build sem cache
    --with-tools    Incluir ferramentas (pgAdmin)

OPÇÕES SYSTEMD:
    --user USER     Usuário para o serviço (padrão: rust-app)
    --install-dir   Diretório de instalação (padrão: /opt/rust-app)

OPÇÕES NIX:
    --target HOST   Host remoto para deploy (ex: user@servidor.com)

EXEMPLOS:
    $0 docker --build
    $0 systemd --user myapp
    $0 nix --target production-server
    $0 check
    $0 cleanup

EOF
}

# ============================================================================
# Verificações
# ============================================================================

check_prerequisites() {
    print_header "Verificando Pré-requisitos"

    local missing=0

    # Verificar Git
    if command -v git &> /dev/null; then
        print_success "Git instalado: $(git --version)"
    else
        print_error "Git não encontrado"
        missing=1
    fi

    # Verificar Rust/Cargo
    if command -v cargo &> /dev/null; then
        print_success "Cargo instalado: $(cargo --version)"
    else
        print_warning "Cargo não encontrado (necessário para build local)"
    fi

    # Verificar Docker (se comando docker for usado)
    if command -v docker &> /dev/null; then
        print_success "Docker instalado: $(docker --version)"
    else
        print_warning "Docker não encontrado (necessário para deploy docker)"
    fi

    # Verificar Docker Compose
    if command -v docker compose &> /dev/null; then
        print_success "Docker Compose instalado"
    else
        print_warning "Docker Compose não encontrado"
    fi

    # Verificar Nix
    if command -v nix &> /dev/null; then
        print_success "Nix instalado: $(nix --version)"
    else
        print_warning "Nix não encontrado (necessário para deploy nix)"
    fi

    echo ""

    if [ $missing -eq 0 ]; then
        print_success "Todos os pré-requisitos básicos estão instalados!"
        return 0
    else
        print_error "Alguns pré-requisitos obrigatórios estão faltando!"
        return 1
    fi
}

# ============================================================================
# Deploy com Docker
# ============================================================================

deploy_docker() {
    print_header "Deploy com Docker Compose"

    local BUILD_FLAG=""
    local CACHE_FLAG=""
    local PROFILE=""

    # Processar opções
    while [[ $# -gt 0 ]]; do
        case $1 in
            --build)
                BUILD_FLAG="--build"
                shift
                ;;
            --no-cache)
                CACHE_FLAG="--no-cache"
                shift
                ;;
            --with-tools)
                PROFILE="--profile tools"
                shift
                ;;
            *)
                shift
                ;;
        esac
    done

    cd "$PROJECT_DIR"

    # Verificar se .env existe
    if [ ! -f ".env" ]; then
        print_warning "Arquivo .env não encontrado"
        print_step "Criando .env a partir de .env.example..."
        cp .env.example .env
        print_success ".env criado! Por favor, configure as variáveis antes de continuar."
        print_info "Edite o arquivo .env e execute o deploy novamente."
        exit 0
    fi

    print_step "Parando containers existentes..."
    docker compose down || true

    print_step "Iniciando serviços..."
    docker compose up -d $BUILD_FLAG $CACHE_FLAG $PROFILE

    echo ""
    print_success "Deploy concluído!"
    echo ""
    print_info "Serviços disponíveis:"
    docker compose ps

    echo ""
    print_info "Comandos úteis:"
    echo "  Ver logs:        docker compose logs -f"
    echo "  Ver logs app:    docker compose logs -f app"
    echo "  Parar:          docker compose down"
    echo "  Reiniciar:      docker compose restart"
    echo "  Entrar no app:  docker compose exec app sh"
}

# ============================================================================
# Deploy com systemd
# ============================================================================

deploy_systemd() {
    print_header "Deploy com systemd"

    local USER="rust-app"
    local INSTALL_DIR="/opt/rust-app"

    # Processar opções
    while [[ $# -gt 0 ]]; do
        case $1 in
            --user)
                USER="$2"
                shift 2
                ;;
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            *)
                shift
                ;;
        esac
    done

    # Verificar se é root
    if [ "$EUID" -ne 0 ]; then
        print_error "Este comando precisa ser executado como root (use sudo)"
        exit 1
    fi

    cd "$PROJECT_DIR"

    print_step "Criando usuário $USER (se não existir)..."
    if ! id "$USER" &>/dev/null; then
        useradd -r -s /bin/false "$USER"
        print_success "Usuário $USER criado"
    else
        print_info "Usuário $USER já existe"
    fi

    print_step "Criando diretórios..."
    mkdir -p "$INSTALL_DIR"/{bin,data,migrations}
    mkdir -p /etc/rust-app

    print_step "Compilando aplicação..."
    cargo build --release --features postgres

    print_step "Instalando binário..."
    cp target/release/rust-app-exemplo "$INSTALL_DIR/bin/"
    chmod +x "$INSTALL_DIR/bin/rust-app-exemplo"

    print_step "Copiando migrations..."
    cp -r migrations/* "$INSTALL_DIR/migrations/"

    print_step "Configurando permissões..."
    chown -R "$USER:$USER" "$INSTALL_DIR"

    print_step "Instalando serviço systemd..."
    cp deploy/rust-app.service /etc/systemd/system/

    # Substituir variáveis no service file
    sed -i "s|User=rust-app|User=$USER|g" /etc/systemd/system/rust-app.service
    sed -i "s|Group=rust-app|Group=$USER|g" /etc/systemd/system/rust-app.service
    sed -i "s|WorkingDirectory=/opt/rust-app|WorkingDirectory=$INSTALL_DIR|g" /etc/systemd/system/rust-app.service
    sed -i "s|/opt/rust-app/bin|$INSTALL_DIR/bin|g" /etc/systemd/system/rust-app.service

    print_step "Criando arquivo de configuração..."
    if [ ! -f "/etc/rust-app/env" ]; then
        cat > /etc/rust-app/env << 'ENVEOF'
RUST_LOG=info
PGHOST=localhost
PGPORT=5432
PGDATABASE=rust_app_db
PGUSER=rust_app_user
ENVEOF
        print_success "Arquivo de configuração criado em /etc/rust-app/env"
    else
        print_info "Arquivo de configuração já existe"
    fi

    print_step "Recarregando systemd..."
    systemctl daemon-reload

    print_step "Habilitando serviço..."
    systemctl enable rust-app.service

    print_step "Iniciando serviço..."
    systemctl start rust-app.service

    echo ""
    print_success "Deploy concluído!"
    echo ""
    print_info "Status do serviço:"
    systemctl status rust-app.service --no-pager

    echo ""
    print_info "Comandos úteis:"
    echo "  Status:    systemctl status rust-app"
    echo "  Logs:      journalctl -u rust-app -f"
    echo "  Parar:     systemctl stop rust-app"
    echo "  Reiniciar: systemctl restart rust-app"
    echo "  Desativar: systemctl disable rust-app"
}

# ============================================================================
# Deploy com Nix
# ============================================================================

deploy_nix() {
    print_header "Deploy com Nix"

    local TARGET=""

    # Processar opções
    while [[ $# -gt 0 ]]; do
        case $1 in
            --target)
                TARGET="$2"
                shift 2
                ;;
            *)
                shift
                ;;
        esac
    done

    cd "$PROJECT_DIR"

    if [ -z "$TARGET" ]; then
        # Build local
        print_step "Fazendo build local com Nix..."
        nix --extra-experimental-features 'nix-command flakes' build

        print_success "Build concluído!"
        print_info "Binário disponível em: ./result/bin/rust-app-exemplo"

        echo ""
        print_info "Para testar:"
        echo "  ./result/bin/rust-app-exemplo --help"
    else
        # Deploy remoto
        print_step "Deploy remoto para $TARGET..."

        print_step "Fazendo build..."
        nix --extra-experimental-features 'nix-command flakes' build

        print_step "Copiando para servidor remoto..."
        nix --extra-experimental-features 'nix-command flakes' copy --to "ssh://$TARGET" ./result

        print_step "Ativando no servidor..."
        ssh "$TARGET" "sudo systemctl restart rust-app || true"

        print_success "Deploy remoto concluído!"
    fi
}

# ============================================================================
# Deploy Local
# ============================================================================

deploy_local() {
    print_header "Deploy Local (Build e Instalação)"

    cd "$PROJECT_DIR"

    print_step "Compilando aplicação em modo release..."
    cargo build --release --features postgres

    print_step "Executando testes..."
    cargo test --features postgres

    print_step "Instalando localmente..."
    cargo install --path . --features postgres --force

    echo ""
    print_success "Deploy local concluído!"
    print_info "Binário instalado em: ~/.cargo/bin/rust-app-exemplo"

    echo ""
    print_info "Para executar:"
    echo "  rust-app-exemplo --help"
}

# ============================================================================
# Cleanup
# ============================================================================

cleanup() {
    print_header "Limpeza de Recursos"

    cd "$PROJECT_DIR"

    print_step "Limpando containers Docker..."
    docker compose down -v 2>/dev/null || print_info "Nenhum container Docker encontrado"

    print_step "Removendo imagens antigas..."
    docker image prune -f || true

    print_step "Limpando build artifacts do Cargo..."
    cargo clean

    print_step "Removendo result do Nix..."
    rm -rf result result-*

    print_success "Limpeza concluída!"
}

# ============================================================================
# Main
# ============================================================================

main() {
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi

    local COMMAND="$1"
    shift

    case "$COMMAND" in
        docker)
            deploy_docker "$@"
            ;;
        systemd)
            deploy_systemd "$@"
            ;;
        nix)
            deploy_nix "$@"
            ;;
        local)
            deploy_local
            ;;
        check)
            check_prerequisites
            ;;
        cleanup)
            cleanup
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Comando desconhecido: $COMMAND"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Executar
main "$@"
