# Installation — SPECTER-NET

## Requisitos

- Linux (kernel 5.4+)
- Rust 1.70+
- PostgreSQL 14+
- SoapySDR (para hardware SDR real)

## Compilação

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clonar repositório
git clone https://github.com/n6c-cybernetics/n6c-pub.git
cd n6c-pub

# Compilar
cargo build --release
```

## Binários

| Binário | Descrição |
|---------|-----------|
| `specter-core` | Servidor central |
| `specter-cli` | Interface de linha de comando |

## Instalação do Sistema

```bash
# Criar usuário
sudo useradd -r -s /bin/false specter

# Instalar binários
sudo cp target/release/specter-core /usr/local/bin/
sudo cp target/release/specter-cli /usr/local/bin/

# Criar estrutura
sudo mkdir -p /etc/specter-net
sudo mkdir -p /var/lib/specter-net
sudo mkdir -p /var/log/specter-net
sudo mkdir -p /var/lib/specter-net/backups

# Copiar configuração
sudo cp config/specter.toml /etc/specter-net/specter.toml

# Permissões
sudo chown -R specter:specter /var/lib/specter-net
sudo chown -R specter:specter /var/log/specter-net
sudo chmod 600 /etc/specter-net/specter.toml
```

## Banco de Dados

```bash
# Criar banco
sudo -u postgres createdb specter_net
sudo -u postgres psql -c "CREATE USER specter WITH PASSWORD 'alterar_senha';"
sudo -u postgres psql -c "GRANT ALL ON DATABASE specter_net TO specter;"

# Aplicar schema
psql -U specter -d specter_net -f migrations/001_init.sql
```

## Serviços Systemd

```bash
# Instalar serviços
sudo cp packaging/systemd/specter-core.service /etc/systemd/system/
sudo cp packaging/systemd/specter-sensor.service /etc/systemd/system/
sudo cp packaging/systemd/specter-radio-agent.service /etc/systemd/system/

# Reload e habilitar
sudo systemctl daemon-reload
sudo systemctl enable specter-core

# Iniciar
sudo systemctl start specter-core

# Verificar
sudo systemctl status specter-core
specter-cli status
```

## Configuração

Editar `/etc/specter-net/specter.toml`:

```toml
[system]
site_name = "MEU-SITE"
log_level = "info"

[rf]
device = "soapy"
sample_rate = 2400000
center_frequency_hz = 433000000
fft_size = 4096

[policy]
automatic_channel_change = true
minimum_confidence = 0.85
cooldown_seconds = 120
max_changes_per_hour = 4

[database]
url = "postgres://specter:senha@localhost/specter_net"

[server]
bind_address = "0.0.0.0"
port = 8080
```

## Verificação

```bash
# Status do sistema
specter-cli status

# Ver configuração
specter-cli config

# Validar configuração
specter-cli validate
```

## Atualização

```bash
# Parar serviços
sudo systemctl stop specter-core specter-sensor specter-radio-agent

# Compilar nova versão
cargo build --release

# Substituir binários
sudo cp target/release/specter-core /usr/local/bin/
sudo cp target/release/specter-cli /usr/local/bin/

# Reiniciar
sudo systemctl start specter-core
```

## Backup

```bash
# Backup do banco
pg_dump -U specter specter_net > backup_$(date +%Y%m%d).sql

# Backup da configuração
sudo cp /etc/specter-net/specter.toml /etc/specter-net/specter.toml.bak
```
