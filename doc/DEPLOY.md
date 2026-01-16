# 🚀 Guia de Deploy - Rust Application

Documentação completa para deploy da aplicação Rust com PostgreSQL em diferentes ambientes.

## 📋 Índice

- [Visão Geral](#visão-geral)
- [Pré-requisitos](#pré-requisitos)
- [Deploy com Docker](#deploy-com-docker)
- [Deploy com Docker Compose](#deploy-com-docker-compose)
- [Deploy com systemd](#deploy-com-systemd)
- [Deploy com Nix](#deploy-com-nix)
- [Deploy com Kubernetes](#deploy-com-kubernetes)
- [CI/CD](#cicd)
- [Monitoramento](#monitoramento)
- [Backup e Restauração](#backup-e-restauração)
- [Troubleshooting](#troubleshooting)

## 🎯 Visão Geral

Esta aplicação pode ser deployada de várias formas:

| Método | Complexidade | Uso Recomendado | Reproduzível |
|--------|--------------|-----------------|--------------|
| **Docker Compose** | ⭐ Baixa | Desenvolvimento/Staging | ✅ |
| **systemd** | ⭐⭐ Média | Servidores tradicionais | ❌ |
| **Nix** | ⭐⭐ Média | Ambiente Nix/NixOS | ✅✅✅ |
| **Kubernetes** | ⭐⭐⭐ Alta | Produção em escala | ✅ |

## 🔧 Pré-requisitos

### Geral

```bash
# Verificar pré-requisitos
./deploy/deploy.sh check
```

### Para Docker

- Docker Engine 24+
- Docker Compose v2+

```bash
docker --version
docker compose version
```

### Para systemd

- Sistema Linux com systemd
- PostgreSQL instalado
- Rust toolchain (para compilar)

### Para Nix

- Nix com Flakes habilitado

```bash
nix --version
```

### Para Kubernetes

- kubectl configurado
- Cluster Kubernetes (minikube, k3s, EKS, GKE, etc.)
- Helm (opcional, mas recomendado)

## 🐳 Deploy com Docker

### Build da Imagem

```bash
# Build simples
docker build -t rust-app:latest .

# Build com cache desabilitado
docker build --no-cache -t rust-app:latest .

# Build para plataforma específica
docker build --platform linux/amd64 -t rust-app:latest .
```

### Executar Container

```bash
# Executar com PostgreSQL externo
docker run -d \
  --name rust-app \
  -e PGHOST=host.docker.internal \
  -e PGPORT=5432 \
  -e PGDATABASE=rust_app_db \
  -e PGUSER=rust_app_user \
  -e PGPASSWORD=senha_segura \
  rust-app:latest \
  db init

# Ver logs
docker logs -f rust-app

# Executar comandos no container
docker exec -it rust-app /app/rust-app-exemplo db list-users
```

### Push para Registry

```bash
# Docker Hub
docker tag rust-app:latest seuusuario/rust-app:latest
docker push seuusuario/rust-app:latest

# GitHub Container Registry
docker tag rust-app:latest ghcr.io/seuusuario/rust-app:latest
docker push ghcr.io/seuusuario/rust-app:latest

# AWS ECR
aws ecr get-login-password --region us-east-1 | \
  docker login --username AWS --password-stdin 123456789.dkr.ecr.us-east-1.amazonaws.com
docker tag rust-app:latest 123456789.dkr.ecr.us-east-1.amazonaws.com/rust-app:latest
docker push 123456789.dkr.ecr.us-east-1.amazonaws.com/rust-app:latest
```

## 🐙 Deploy com Docker Compose

### Configuração Inicial

```bash
# 1. Copiar arquivo de exemplo
cp .env.example .env

# 2. Editar variáveis de ambiente
nano .env

# 3. Configurar senhas (IMPORTANTE!)
# Edite POSTGRES_PASSWORD no .env
```

### Iniciar Serviços

```bash
# Usando script de deploy
./deploy/deploy.sh docker

# Ou manualmente
docker compose up -d

# Com rebuild das imagens
docker compose up -d --build

# Incluir ferramentas (pgAdmin)
docker compose --profile tools up -d
```

### Gerenciar Serviços

```bash
# Ver status
docker compose ps

# Ver logs
docker compose logs -f
docker compose logs -f app
docker compose logs -f postgres

# Parar serviços
docker compose stop

# Reiniciar serviços
docker compose restart

# Parar e remover
docker compose down

# Parar e remover volumes (CUIDADO!)
docker compose down -v
```

### Executar Comandos

```bash
# Entrar no container
docker compose exec app sh

# Executar migrations
docker compose exec app /app/rust-app-exemplo db init

# Listar usuários
docker compose exec app /app/rust-app-exemplo db list-users

# Criar usuário
docker compose exec app /app/rust-app-exemplo db create-user "Alice" "alice@example.com"
```

### Acessar Serviços

- **Aplicação**: http://localhost:8080
- **PostgreSQL**: localhost:5432
- **PgAdmin** (se --profile tools): http://localhost:5050

## ⚙️ Deploy com systemd

### Instalação Automática

```bash
# Executar como root
sudo ./deploy/deploy.sh systemd

# Com opções customizadas
sudo ./deploy/deploy.sh systemd --user myapp --install-dir /opt/myapp
```

### Instalação Manual

```bash
# 1. Criar usuário
sudo useradd -r -s /bin/false rust-app

# 2. Criar diretórios
sudo mkdir -p /opt/rust-app/{bin,data,migrations}
sudo mkdir -p /etc/rust-app

# 3. Compilar aplicação
cargo build --release --features postgres

# 4. Copiar binário
sudo cp target/release/rust-app-exemplo /opt/rust-app/bin/
sudo chmod +x /opt/rust-app/bin/rust-app-exemplo

# 5. Copiar migrations
sudo cp -r migrations/* /opt/rust-app/migrations/

# 6. Configurar permissões
sudo chown -R rust-app:rust-app /opt/rust-app

# 7. Criar arquivo de configuração
sudo tee /etc/rust-app/env << 'EOF'
RUST_LOG=info
PGHOST=localhost
PGPORT=5432
PGDATABASE=rust_app_db
PGUSER=rust_app_user
PGPASSWORD=senha_segura
EOF

# 8. Instalar service
sudo cp deploy/rust-app.service /etc/systemd/system/

# 9. Recarregar systemd
sudo systemctl daemon-reload

# 10. Habilitar e iniciar
sudo systemctl enable rust-app
sudo systemctl start rust-app
```

### Gerenciar Serviço

```bash
# Ver status
sudo systemctl status rust-app

# Ver logs
sudo journalctl -u rust-app -f

# Parar serviço
sudo systemctl stop rust-app

# Reiniciar serviço
sudo systemctl restart rust-app

# Desabilitar serviço
sudo systemctl disable rust-app

# Ver logs com filtro
sudo journalctl -u rust-app --since "1 hour ago"
sudo journalctl -u rust-app --since "2024-01-01"
```

### Atualização

```bash
# 1. Compilar nova versão
cargo build --release --features postgres

# 2. Parar serviço
sudo systemctl stop rust-app

# 3. Substituir binário
sudo cp target/release/rust-app-exemplo /opt/rust-app/bin/

# 4. Executar migrations (se houver)
sudo -u rust-app /opt/rust-app/bin/rust-app-exemplo db init

# 5. Reiniciar serviço
sudo systemctl start rust-app
```

## ❄️ Deploy com Nix

### Build Local

```bash
# Usar script de deploy
./deploy/deploy.sh nix

# Ou manualmente
nix --extra-experimental-features 'nix-command flakes' build

# Testar binário
./result/bin/rust-app-exemplo --help
```

### Deploy Remoto

```bash
# Para servidor com Nix instalado
./deploy/deploy.sh nix --target user@servidor.com

# Ou manualmente
# 1. Build
nix --extra-experimental-features 'nix-command flakes' build

# 2. Copiar para servidor
nix --extra-experimental-features 'nix-command flakes' copy \
  --to ssh://user@servidor.com \
  ./result

# 3. Ativar no servidor
ssh user@servidor.com "sudo systemctl restart rust-app"
```

### NixOS Configuration

Para NixOS, adicione ao `configuration.nix`:

```nix
{ config, pkgs, ... }:

{
  # Importar flake
  imports = [ 
    (builtins.getFlake "path/to/rust-app").nixosModules.default
  ];

  # Configurar serviço
  services.rust-app = {
    enable = true;
    database = {
      host = "localhost";
      port = 5432;
      name = "rust_app_db";
      user = "rust_app_user";
    };
  };
}
```

## ☸️ Deploy com Kubernetes

### Preparação

```bash
# 1. Build e push da imagem
docker build -t seuusuario/rust-app:v1.0.0 .
docker push seuusuario/rust-app:v1.0.0

# 2. Atualizar imagem no deployment
sed -i 's|rust-app:latest|seuusuario/rust-app:v1.0.0|g' deploy/kubernetes/deployment.yaml
```

### Deploy

```bash
# Aplicar configuração
kubectl apply -f deploy/kubernetes/deployment.yaml

# Verificar status
kubectl get all -n rust-app

# Ver pods
kubectl get pods -n rust-app

# Ver logs
kubectl logs -f -n rust-app deployment/rust-app

# Executar comando no pod
kubectl exec -it -n rust-app deployment/rust-app -- /app/rust-app-exemplo db list-users
```

### Gerenciar

```bash
# Scale horizontal
kubectl scale deployment rust-app -n rust-app --replicas=3

# Ver logs de um pod específico
kubectl logs -f -n rust-app pod/rust-app-xxx

# Port forward para acesso local
kubectl port-forward -n rust-app service/rust-app-service 8080:80

# Deletar tudo
kubectl delete namespace rust-app
```

### Helm Chart (Opcional)

Para criar um Helm chart:

```bash
# Criar chart
helm create rust-app-chart

# Instalar
helm install rust-app ./rust-app-chart \
  --namespace rust-app \
  --create-namespace \
  --set image.tag=v1.0.0

# Atualizar
helm upgrade rust-app ./rust-app-chart \
  --namespace rust-app \
  --set image.tag=v1.1.0

# Desinstalar
helm uninstall rust-app --namespace rust-app
```

## 🔄 CI/CD

### GitHub Actions

Crie `.github/workflows/deploy.yml`:

```yaml
name: Build and Deploy

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build
        run: cargo build --release --features postgres
      
      - name: Test
        run: cargo test --features postgres
      
      - name: Build Docker Image
        run: docker build -t rust-app:${{ github.sha }} .
      
      - name: Push to Registry
        run: |
          echo ${{ secrets.DOCKER_PASSWORD }} | docker login -u ${{ secrets.DOCKER_USERNAME }} --password-stdin
          docker tag rust-app:${{ github.sha }} ${{ secrets.DOCKER_USERNAME }}/rust-app:latest
          docker push ${{ secrets.DOCKER_USERNAME }}/rust-app:latest

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Deploy to Server
        uses: appleboy/ssh-action@master
        with:
          host: ${{ secrets.SERVER_HOST }}
          username: ${{ secrets.SERVER_USER }}
          key: ${{ secrets.SSH_PRIVATE_KEY }}
          script: |
            cd /opt/rust-app
            docker compose pull
            docker compose up -d
```

### GitLab CI

Crie `.gitlab-ci.yml`:

```yaml
stages:
  - build
  - test
  - deploy

build:
  stage: build
  image: rust:latest
  script:
    - cargo build --release --features postgres
  artifacts:
    paths:
      - target/release/rust-app-exemplo

test:
  stage: test
  image: rust:latest
  script:
    - cargo test --features postgres

deploy:
  stage: deploy
  only:
    - main
  script:
    - docker build -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA .
    - docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
    - ssh user@server "cd /opt/app && docker compose up -d"
```

## 📊 Monitoramento

### Logs

```bash
# Docker Compose
docker compose logs -f --tail=100 app

# systemd
sudo journalctl -u rust-app -f

# Kubernetes
kubectl logs -f -n rust-app deployment/rust-app
```

### Prometheus + Grafana

Adicione ao seu código (exemplo):

```rust
// Adicionar ao Cargo.toml:
// prometheus = "0.13"

use prometheus::{Counter, Encoder, TextEncoder};

lazy_static! {
    static ref DB_QUERIES: Counter = 
        Counter::new("db_queries_total", "Total DB queries").unwrap();
}

// Endpoint de métricas
async fn metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
```

### Health Checks

```bash
# Docker
docker compose exec app /app/rust-app-exemplo db ping

# systemd
/opt/rust-app/bin/rust-app-exemplo db ping

# Kubernetes
kubectl exec -n rust-app deployment/rust-app -- /app/rust-app-exemplo db ping
```

## 💾 Backup e Restauração

### Backup PostgreSQL

```bash
# Docker Compose
docker compose exec postgres pg_dump -U rust_app_user rust_app_db > backup.sql

# systemd
sudo -u postgres pg_dump rust_app_db > backup.sql

# Kubernetes
kubectl exec -n rust-app deployment/postgres -- \
  pg_dump -U rust_app_user rust_app_db > backup.sql
```

### Restauração

```bash
# Docker Compose
cat backup.sql | docker compose exec -T postgres psql -U rust_app_user rust_app_db

# systemd
sudo -u postgres psql rust_app_db < backup.sql

# Kubernetes
kubectl exec -i -n rust-app deployment/postgres -- \
  psql -U rust_app_user rust_app_db < backup.sql
```

### Backup Automatizado

Adicione ao crontab:

```bash
# Backup diário às 2h da manhã
0 2 * * * /opt/rust-app/scripts/backup.sh
```

Crie `backup.sh`:

```bash
#!/bin/bash
BACKUP_DIR=/opt/rust-app/backups
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump -U rust_app_user rust_app_db > $BACKUP_DIR/backup_$DATE.sql
find $BACKUP_DIR -name "backup_*.sql" -mtime +7 -delete
```

## 🔧 Troubleshooting

### Problemas Comuns

#### 1. Aplicação não conecta ao banco

```bash
# Verificar se PostgreSQL está rodando
docker compose ps postgres
sudo systemctl status postgresql

# Verificar variáveis de ambiente
docker compose exec app env | grep PG
echo $PGHOST $PGPORT $PGDATABASE

# Testar conexão manualmente
psql -h localhost -p 5432 -U rust_app_user -d rust_app_db
```

#### 2. Migrations falham

```bash
# Ver tabela de migrations
psql -h localhost -U rust_app_user -d rust_app_db \
  -c "SELECT * FROM _sqlx_migrations;"

# Executar migrations manualmente
docker compose exec app /app/rust-app-exemplo db init
```

#### 3. Container não inicia

```bash
# Ver logs detalhados
docker compose logs app

# Verificar health check
docker inspect rust-app | grep -A 10 Health

# Entrar no container
docker compose exec app sh
```

#### 4. Performance lenta

```bash
# Ver queries lentas no PostgreSQL
psql -h localhost -U rust_app_user -d rust_app_db << 'EOF'
SELECT query, calls, total_time, mean_time 
FROM pg_stat_statements 
ORDER BY mean_time DESC 
LIMIT 10;
EOF

# Ver uso de recursos
docker stats rust-app
kubectl top pods -n rust-app
```

### Debug Mode

```bash
# Habilitar logs debug
export RUST_LOG=debug
docker compose up -d

# Ou no arquivo .env
RUST_LOG=debug
```

## 🔐 Segurança

### Boas Práticas

1. **Nunca commite senhas no Git**
   - Use `.env` (já está no `.gitignore`)
   - Use secrets do Kubernetes
   - Use AWS Secrets Manager / HashiCorp Vault

2. **Use HTTPS em produção**
   - Configure reverse proxy (nginx/traefik)
   - Use Let's Encrypt para certificados

3. **Limite permissões**
   - Execute como usuário não-root
   - Use AppArmor/SELinux

4. **Atualize regularmente**
   ```bash
   cargo audit
   docker pull postgres:16-alpine
   ```

### Secrets Management

#### Kubernetes

```bash
# Criar secret
kubectl create secret generic rust-app-secret \
  --from-literal=PGPASSWORD=senha_super_segura \
  -n rust-app

# Usar no deployment (já configurado)
```

#### Docker Compose com Secrets

```yaml
secrets:
  postgres_password:
    file: ./secrets/postgres_password.txt

services:
  app:
    secrets:
      - postgres_password
    environment:
      PGPASSWORD_FILE: /run/secrets/postgres_password
```

## 📚 Recursos Adicionais

- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [systemd Service File](https://www.freedesktop.org/software/systemd/man/systemd.service.html)
- [Nix Deployment](https://nixos.wiki/wiki/Deployment)

## 🆘 Suporte

Para problemas ou dúvidas:

1. Verifique os logs
2. Consulte este guia
3. Abra uma issue no repositório

---

**🎉 Deploy bem-sucedido! Sua aplicação Rust está pronta para produção!**