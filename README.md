# Data-Scorched Safe

**N6 Cybernetics — Hardened Device Protection System**

Sistema real de proteção contra adulteração, extração física e comprometimento de dispositivos em ambientes hostis.

---

## Visão Geral

O **Data-Scorched Safe** é um sistema de segurança desenvolvido pela N6 Cybernetics para proteção de dispositivos móveis endurecidos, equipamentos embarcados e servidores instalados em ambientes de alto risco físico.

O sistema monitora sinais físicos e lógicos de adulteração em tempo real, aplica respostas defensivas automáticas e preserva evidências para investigação — sem nunca executar ações destrutivas ou anti-forenses.

## Arquitetura

```
┌─────────────────────────────────────────────────────┐
│                  data-scorched-agent                 │
│              (Daemon principal — Rust)                │
├──────────┬──────────┬──────────┬──────────┬─────────┤
│ hardware │integrity │  policy  │ response │  audit   │
│ monitor  │  engine  │  engine  │  engine  │   log    │
├──────────┴──────────┴──────────┴──────────┴─────────┤
│                   crypto-manager                     │
│              (AES-256-GCM + HMAC-SHA256)             │
└─────────────────────────────────────────────────────┘
```

### Máquina de Estados

```
NORMAL ──→ ELEVATED ──→ LOCKDOWN ──→ QUARANTINE ──→ RECOVERY ──→ NORMAL
```

| Estado | Descrição |
|--------|-----------|
| **NORMAL** | Operação regular |
| **ELEVATED** | Eventos incomuns detectados |
| **LOCKDOWN** | Dispositivo bloqueado, sessões encerradas |
| **QUARANTINE** | Serviços não essenciais desativados |
| **RECOVERY** | Aguardando autenticação administrativa |

## Componentes

| Crate | Descrição |
|-------|-----------|
| `ds-common` | Tipos compartilhados, configuração, erros, eventos |
| `ds-hardware` | Monitoramento de USB, temperatura, chassis, TPM |
| `ds-integrity` | Verificação de Secure Boot, TPM 2.0, kernel, módulos |
| `ds-policy` | Engine de risco determinístico e política |
| `ds-crypto` | Criptografia AES-256-GCM, HMAC-SHA256, chaves |
| `ds-response` | Respostas defensivas automáticas |
| `ds-audit` | Logs append-only com hash chain |
| `ds-agent` | Daemon + CLI `datascorched` |

## Sensores Suportados

| Sensor | Fonte | Plataforma |
|--------|-------|------------|
| Temperatura | `/sys/class/hwmon`, `/sys/class/thermal` | Linux |
| USB | `/sys/bus/usb/devices` | Linux |
| TPM 2.0 | `/dev/tpm0` | Linux |
| Secure Boot | EFI variables | Linux UEFI |
| Chassis | IPMI / sysfs | Variável |
| Kernel modules | `/proc/modules` | Linux |

## Compilação

### Requisitos

- Rust 1.70+ (instalar via [rustup](https://rustup.rs))
- Linux (kernel 5.4+)
- UEFI com Secure Boot (opcional)
- TPM 2.0 (opcional)

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Compilar
cargo build --release

# Binário resultante
target/release/datascorched
```

## Instalação

```bash
# Instalar binário
sudo cp target/release/datascorched /usr/local/bin/

# Criar estrutura de configuração
sudo mkdir -p /etc/datascorched
sudo cp config/config.example.toml /etc/datascorched/config.toml

# Criar diretório de logs
sudo mkdir -p /var/log/datascorched

# Instalar e habilitar serviço
sudo cp packaging/systemd/datascorched.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable datascorched
sudo systemctl start datascorched
```

## Uso

```bash
# Status do sistema
datascorched status

# Listar sensores e dispositivos
datascorched sensors

# Verificar integridade do sistema
datascorched integrity

# Visualizar eventos de auditoria
datascorched events

# Ativar lockdown manual
datascorched lockdown

# Iniciar recuperação
datascorched recover

# Validar configuração
datascorched policy validate

# Verificar integridade dos logs
datascorched audit verify
```

## Configuração

Arquivo: `/etc/datascorched/config.toml`

```toml
[security]
secure_boot_required = true
tpm_required = true
lock_on_tamper = true
network_quarantine = true
lock_timeout_secs = 300

[risk]
elevated_threshold = 30
lockdown_threshold = 60
quarantine_threshold = 80

[audit]
remote_logging = false
hash_chain = true

[sensors]
monitor_temperature = true
monitor_usb = true
monitor_pcie = true
monitor_chassis = true
```

## Princípios de Segurança

1. **Defesa em Profundidade** — Múltiplas camadas de proteção
2. **Menor Privilégio** — Serviço roda com permissões mínimas
3. **Fail-Safe** — Falhas preservam dados e evidências
4. **Sem Ações Destrutivas** — Nunca sobrescreve ou apaga dados automaticamente
5. **Auditoria Completa** — Todos os eventos são registrados com hash chain
6. **Recuperação Garantida** — Sempre permitir restauração administrativa

## Limitações

O sistema **não** protege contra:
- Atacantes com acesso físico prolongado e equipamento de laboratório
- Comprometimento de firmware não detectável
- Vulnerabilidades de hardware (spectre, meltdown, etc.)
- Chaves criptográficas já expostas antes da ativação
- Ataques anteriores à instalação do sistema

## Licença

MIT — Ver [LICENSE](LICENSE)

## Autor

**Joaquim Pedro de Morais Filho** — N6 Cybernetics

---

*N6 Cybernetics — Hardened Security Systems*
