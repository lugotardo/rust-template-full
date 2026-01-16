# Dockerfile multi-stage para aplicação Rust com PostgreSQL

# ============================================================================
# Stage 1: Builder - Compilar a aplicação
# ============================================================================
FROM rust:1.75-slim as builder

# Instalar dependências de build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Criar diretório de trabalho
WORKDIR /app

# Copiar manifests
COPY Cargo.toml Cargo.lock ./

# Copiar código fonte
COPY src ./src
COPY benches ./benches
COPY tests ./tests
COPY migrations ./migrations

# Build da aplicação em modo release
# Compilar com feature postgres
RUN cargo build --release --features postgres

# ============================================================================
# Stage 2: Runtime - Imagem final otimizada
# ============================================================================
FROM debian:bookworm-slim

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -m -u 1000 appuser

# Criar diretórios necessários
RUN mkdir -p /app/migrations && chown -R appuser:appuser /app

# Mudar para usuário não-root
USER appuser
WORKDIR /app

# Copiar binário compilado do stage builder
COPY --from=builder --chown=appuser:appuser /app/target/release/rust-app-exemplo /app/rust-app-exemplo

# Copiar migrations
COPY --chown=appuser:appuser migrations ./migrations

# Expor porta (se necessário para futuras features web)
EXPOSE 8080

# Variáveis de ambiente padrão
ENV RUST_LOG=info
ENV PGHOST=localhost
ENV PGPORT=5432
ENV PGDATABASE=rust_app_db
ENV PGUSER=rust_app_user

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/rust-app-exemplo", "db", "ping"] || exit 1

# Comando padrão
ENTRYPOINT ["/app/rust-app-exemplo"]
CMD ["--help"]
