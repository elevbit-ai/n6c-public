# SPECTER-NET

**N6 Cybernetics — Defensive RF Spectrum Monitoring & Communication Resilience**

Sistema defensivo de monitoramento de espectro RF e resiliência automática de comunicações autorizadas.

---

## Visão Geral

O **SPECTER-NET** monitora continuamente o espectro de radiofrequência, detecta interferência e jamming, e aplica mudanças automáticas de canal em rádios autorizados para manter comunicações disponíveis.

## Arquitetura

```
┌─────────────────────────────────────────────────────────┐
│                    specter-console                       │
│                (Dashboard React + TypeScript)             │
├─────────────────────────────────────────────────────────┤
│                     specter-core                         │
│           (Servidor Central — Rust + Axum)                │
├─────────────┬─────────────┬─────────────┬───────────────┤
│  specter-   │  specter-   │  specter-   │   specter-    │
│  detector   │   policy    │    core     │  radio-agent  │
│             │             │   (API)     │               │
├─────────────┴─────────────┴─────────────┴───────────────┤
│                    specter-sensor                         │
│           (Daemon — Rust + Tokio + SoapySDR)              │
├─────────────────────────────────────────────────────────┤
│                  Hardware SDR (RTL-SDR / USRP)            │
└─────────────────────────────────────────────────────────┘
```

### Fluxo de Dados

```
SDR Hardware
  -> Driver (SoapySDR)
  -> Buffer Circular
  -> FFT
  -> PSD
  -> Noise Floor Estimation
  -> Channel Occupancy
  -> Anomaly Detection
  -> Event Engine
  -> API / Dashboard
```

## Componentes

| Crate | Descrição |
|-------|-----------|
| `specter-common` | Tipos compartilhados, configuração, erros |
| `specter-dsp` | Pipeline DSP: FFT, PSD, noise floor, ocupação |
| `specter-sdr` | Camada de abstração de hardware SDR |
| `specter-detector` | Motor de detecção de interferência/jamming |
| `specter-policy` | Engine de decisão, allowlist, políticas |
| `specter-core` | Servidor central, API REST, correlação |
| `specter-sensor` | Daemon do sensor RF |
| `specter-radio-agent` | Agente de controle de rádios autorizados |

## Compilação

### Requisitos

- Rust 1.70+ ([rustup](https://rustup.rs))
- PostgreSQL 14+
- SoapySDR (opcional, para hardware real)

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Compilar
cargo build --release

# Binários resultantes
target/release/specter-core
target/release/specter-cli
```

## Instalação

```bash
# Criar usuário do sistema
sudo useradd -r -s /bin/false specter

# Instalar binários
sudo cp target/release/specter-core /usr/local/bin/
sudo cp target/release/specter-cli /usr/local/bin/

# Criar estrutura de configuração
sudo mkdir -p /etc/specter-net
sudo cp config/specter.toml /etc/specter-net/specter.toml

# Criar diretórios
sudo mkdir -p /var/lib/specter-net
sudo mkdir -p /var/log/specter-net

# Instalar serviços
sudo cp packaging/systemd/*.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable specter-core specter-sensor specter-radio-agent

# Criar banco de dados
sudo -u postgres createdb specter_net
sudo -u postgres psql -c "CREATE USER specter WITH PASSWORD 'specter';"
sudo -u postgres psql -c "GRANT ALL ON DATABASE specter_net TO specter;"
psql -U specter -d specter_net -f migrations/001_init.sql

# Iniciar serviços
sudo systemctl start specter-core
```

## Uso

```bash
# Verificar status
specter-cli status

# Ver configuração
specter-cli config

# Validar configuração
specter-cli validate

# Ver versão
specter-cli version
```

## Configuração

Arquivo: `/etc/specter-net/specter.toml`

```toml
[system]
site_name = "LAB-01"
log_level = "info"

[rf]
device = "soapy"
sample_rate = 2400000
center_frequency_hz = 433000000
fft_size = 4096
window_function = "hann"

[policy]
automatic_channel_change = true
minimum_confidence = 0.85
cooldown_seconds = 120
max_changes_per_hour = 4

[security]
require_mtls = true

[database]
url = "postgres://specter:specter@localhost/specter_net"

[server]
bind_address = "0.0.0.0"
port = 8080
```

## API REST

| Endpoint | Método | Descrição |
|----------|--------|-----------|
| `/api/v1/health` | GET | Health check |
| `/api/v1/sensors` | GET | Listar sensores |
| `/api/v1/sensors/{id}` | GET | Detalhes do sensor |
| `/api/v1/spectrum/current` | GET | Espectro atual |
| `/api/v1/events` | GET | Eventos RF |
| `/api/v1/radios` | GET | Rádios gerenciados |
| `/api/v1/radios/{id}/channel-change` | POST | Trocar canal |
| `/api/v1/channels` | GET | Canais disponíveis |
| `/api/v1/alerts` | GET | Alertas |
| `/api/v1/audit` | GET | Log de auditoria |

## Segurança

- TLS 1.3 com mTLS
- RBAC (VIEWER, OPERATOR, ADMIN, AUDITOR)
- Comandos assinados com nonce e timestamp
- Rate limiting
- Validação de allowlist
- Rollback automático em falha
- Logs de auditoria com hash chain

## Limitações

O sistema **não** executa:
- Transmissão de sinais de jamming
- Interferência em redes de terceiros
- Spoofing de dispositivos
- Captura de conteúdo de comunicações
- Mudança de frequência fora de bandas autorizadas

## Licença

MIT — Ver [LICENSE](LICENSE)

## Autor

**Joaquim Pedro de Morais Filho** — N6 Cybernetics

---

*N6 Cybernetics — Defensive RF Systems*
