{
  description = "Aplicação Rust modelo com Nix e PostgreSQL";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Configuração do PostgreSQL
        # Você pode modificar estas variáveis de ambiente no shellHook
        pgVersion = pkgs.postgresql_16;
        pgDataDir = "./.pgdata";
        pgPort = "5432";
        pgDatabase = "rust_app_db";
        pgUser = "rust_app_user";

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        # Dependências adicionais quando PostgreSQL está habilitado
        postgresInputs = with pkgs; [
          pgVersion
        ];

      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rust-app-exemplo";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;
        };

        # Shell padrão sem PostgreSQL
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            cargo-watch
            cargo-edit
            rustfmt
            clippy
          ]);

          shellHook = ''
            echo "🦀 Ambiente Rust com Nix carregado!"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "💡 Para usar PostgreSQL, execute: nix develop .#with-postgres"
          '';
        };

        # Shell com PostgreSQL habilitado
        devShells.with-postgres = pkgs.mkShell {
          buildInputs = buildInputs ++ postgresInputs;

          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            cargo-watch
            cargo-edit
            rustfmt
            clippy
          ]);

          # Variáveis de ambiente para PostgreSQL
          PGDATA = pgDataDir;
          PGPORT = pgPort;
          PGDATABASE = pgDatabase;
          PGUSER = pgUser;

          shellHook = ''
            echo "🦀 Ambiente Rust com Nix + PostgreSQL carregado!"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "🐘 PostgreSQL ${pgVersion.version} configurado!"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

            # Configurações do PostgreSQL
            export PGDATA="${pgDataDir}"
            export PGPORT="${pgPort}"
            export PGDATABASE="${pgDatabase}"
            export PGUSER="${pgUser}"
            export PGHOST="localhost"

            # Criar diretório de dados se não existir
            if [ ! -d "$PGDATA" ]; then
              echo "📁 Criando diretório de dados PostgreSQL..."
              initdb --encoding=UTF8 --locale=C --username=$PGUSER
              echo ""
            fi

            # Função para iniciar o PostgreSQL
            pg_start() {
              if [ -f "$PGDATA/postmaster.pid" ]; then
                echo "⚠️  PostgreSQL já está rodando!"
                return 1
              fi

              echo "🚀 Iniciando PostgreSQL..."
              pg_ctl -D "$PGDATA" -l "$PGDATA/logfile" -o "-k /tmp -p $PGPORT" start

              # Aguardar PostgreSQL iniciar
              sleep 2

              # Criar banco de dados se não existir
              if ! psql -lqt | cut -d \| -f 1 | grep -qw "$PGDATABASE"; then
                echo "📊 Criando banco de dados '$PGDATABASE'..."
                createdb "$PGDATABASE"
              fi

              echo ""
              echo "✅ PostgreSQL iniciado com sucesso!"
              echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
              echo "📋 Informações de Conexão:"
              echo "   Host:     localhost"
              echo "   Porta:    $PGPORT"
              echo "   Banco:    $PGDATABASE"
              echo "   Usuário:  $PGUSER"
              echo ""
              echo "🔗 String de conexão:"
              echo "   postgresql://$PGUSER@localhost:$PGPORT/$PGDATABASE"
              echo ""
              echo "🔧 Comandos disponíveis:"
              echo "   pg_start    - Iniciar PostgreSQL"
              echo "   pg_stop     - Parar PostgreSQL"
              echo "   pg_restart  - Reiniciar PostgreSQL"
              echo "   pg_status   - Ver status do PostgreSQL"
              echo "   pg_psql     - Conectar ao banco via psql"
              echo "   pg_reset    - Resetar banco de dados (CUIDADO!)"
              echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            }

            # Função para parar o PostgreSQL
            pg_stop() {
              if [ ! -f "$PGDATA/postmaster.pid" ]; then
                echo "⚠️  PostgreSQL não está rodando!"
                return 1
              fi

              echo "🛑 Parando PostgreSQL..."
              pg_ctl -D "$PGDATA" stop
              echo "✅ PostgreSQL parado!"
            }

            # Função para reiniciar o PostgreSQL
            pg_restart() {
              pg_stop
              sleep 1
              pg_start
            }

            # Função para ver status
            pg_status() {
              pg_ctl -D "$PGDATA" status
            }

            # Função para conectar via psql
            pg_psql() {
              psql -h localhost -p "$PGPORT" -U "$PGUSER" "$PGDATABASE"
            }

            # Função para resetar o banco
            pg_reset() {
              echo "⚠️  ATENÇÃO: Isso vai DELETAR todos os dados!"
              echo -n "Tem certeza? (yes/no): "
              read -r response
              if [ "$response" = "yes" ]; then
                pg_stop
                echo "🗑️  Removendo dados..."
                rm -rf "$PGDATA"
                echo "📁 Recriando banco..."
                initdb --encoding=UTF8 --locale=C --username=$PGUSER
                pg_start
              else
                echo "❌ Operação cancelada."
              fi
            }

            # Exportar funções
            export -f pg_start
            export -f pg_stop
            export -f pg_restart
            export -f pg_status
            export -f pg_psql
            export -f pg_reset

            echo ""
            echo "💡 Execute 'pg_start' para iniciar o PostgreSQL"
            echo ""

            # Auto-start opcional (descomente se quiser iniciar automaticamente)
            # pg_start
          '';
        };
      }
    );
}
