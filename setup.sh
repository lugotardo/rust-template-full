#!/usr/bin/env bash

set -e

echo "🦀 Setup da Aplicação Rust com Nix"
echo "=================================="
echo ""

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para printar com cor
print_step() {
    echo -e "${BLUE}==>${NC} $1"
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

# Verificar se Nix está instalado
print_step "Verificando se Nix está instalado..."
if ! command -v nix &> /dev/null; then
    print_error "Nix não está instalado!"
    echo ""
    echo "Instale Nix com:"
    echo "  sh <(curl -L https://nixos.org/nix/install) --daemon"
    exit 1
fi
print_success "Nix está instalado"

# Verificar se Flakes está habilitado
print_step "Verificando se Nix Flakes está habilitado..."
NIX_CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/nix"
NIX_CONFIG_FILE="$NIX_CONFIG_DIR/nix.conf"

if ! nix --extra-experimental-features 'nix-command flakes' flake --version &> /dev/null; then
    print_warning "Flakes não está habilitado"

    echo ""
    echo "Você quer habilitar Nix Flakes automaticamente? (y/n)"
    read -r response

    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        mkdir -p "$NIX_CONFIG_DIR"

        if [ -f "$NIX_CONFIG_FILE" ]; then
            if ! grep -q "experimental-features" "$NIX_CONFIG_FILE"; then
                echo "experimental-features = nix-command flakes" >> "$NIX_CONFIG_FILE"
                print_success "Flakes habilitado em $NIX_CONFIG_FILE"
            else
                print_success "Flakes já está configurado"
            fi
        else
            echo "experimental-features = nix-command flakes" > "$NIX_CONFIG_FILE"
            print_success "Arquivo de configuração criado: $NIX_CONFIG_FILE"
        fi

        print_warning "Por favor, reinicie o daemon do Nix ou faça logout/login"
        echo "Você pode continuar usando --extra-experimental-features por enquanto"
    fi
else
    print_success "Nix Flakes está habilitado"
fi

# Inicializar Git se necessário
print_step "Verificando repositório Git..."
if [ ! -d ".git" ]; then
    print_warning "Repositório Git não inicializado"
    git init
    print_success "Repositório Git inicializado"
else
    print_success "Repositório Git já existe"
fi

# Adicionar arquivos ao Git
print_step "Adicionando arquivos ao Git..."
git add .
print_success "Arquivos adicionados"

# Gerar Cargo.lock
print_step "Gerando Cargo.lock..."
if [ ! -f "Cargo.lock" ]; then
    # Tentar com Nix primeiro
    if nix --extra-experimental-features 'nix-command flakes' develop --command cargo generate-lockfile 2>/dev/null; then
        print_success "Cargo.lock gerado via Nix"
    else
        print_warning "Não foi possível gerar via Nix, tentando com cargo local..."

        # Verificar se cargo está disponível
        if command -v cargo &> /dev/null; then
            cargo generate-lockfile
            print_success "Cargo.lock gerado via cargo local"
        else
            print_error "Não foi possível gerar Cargo.lock"
            echo "Execute manualmente:"
            echo "  nix --extra-experimental-features 'nix-command flakes' develop --command cargo generate-lockfile"
        fi
    fi
else
    print_success "Cargo.lock já existe"
fi

# Adicionar Cargo.lock ao Git se foi gerado
if [ -f "Cargo.lock" ]; then
    git add Cargo.lock
fi

echo ""
echo "=================================="
echo -e "${GREEN}✓ Setup concluído!${NC}"
echo "=================================="
echo ""
echo "Próximos passos:"
echo ""
echo "1. Entrar no ambiente de desenvolvimento:"
echo -e "   ${BLUE}nix --extra-experimental-features 'nix-command flakes' develop${NC}"
echo ""
echo "2. Ou usar make:"
echo -e "   ${BLUE}make dev${NC}"
echo ""
echo "3. Compilar o projeto:"
echo -e "   ${BLUE}cargo build${NC}"
echo ""
echo "4. Executar a aplicação:"
echo -e "   ${BLUE}cargo run -- --help${NC}"
echo ""
echo "5. Executar testes:"
echo -e "   ${BLUE}cargo test${NC}"
echo ""
echo "6. Build com Nix (reproduzível):"
echo -e "   ${BLUE}nix --extra-experimental-features 'nix-command flakes' build${NC}"
echo ""
echo "Ver todos os comandos disponíveis:"
echo -e "   ${BLUE}make help${NC}"
echo ""
