# Data-Scorched Safe

Sistema real de proteção contra adulteração, extração física e comprometimento de dispositivos.

**Concepção e autoria:** Joaquim Pedro de Morais Filho

## Visão Geral

O Data-Scorched Safe é um sistema de segurança projetado para dispositivos móveis endurecidos, equipamentos embarcados e servidores instalados em ambientes hostis ou de alto risco físico.

O sistema detecta sinais físicos e lógicos de adulteração, protege dados por criptografia, bloqueia acesso em situações de alto risco, preserva evidências e telemetria, e permite recuperação administrativa.

## Características

- **Daemon Rust** com privilégios mínimos
- **Monitoramento de hardware real** (USB, temperatura, chassis, TPM)
- **Engine de integridade** (Secure Boot, TPM 2.0, kernel, módulos)
- **Engine de política** com máquina de estados (NORMAL → ELEVATED → LOCKDOWN → QUARANTINE → RECOVERY)
- **Engine de resposta** com ações automáticas permitidas
- **Logs de auditoria** append-only com hash chain
- **Criptografia** AES-256-GCM + HMAC-SHA256
- **CLI completa** para gerenciamento
- **Serviço systemd** com sandboxing

## Estado do Sistema

```
NORMAL → ELEVATED → LOCKDOWN → QUARANTINE → RECOVERY → NORMAL
```

## Compilação

```bash
# Requisitos
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compilar
cargo build --release

# Binário resultante
target/release/datascorched
```

## Instalação

```bash
# Compilar release
cargo build --release

# Instalar binário
sudo cp target/release/datascorched /usr/local/bin/

# Criar diretório de configuração
sudo mkdir -p /etc/datascorched
sudo cp config/config.example.toml /etc/datascorched/config.toml

# Criar diretório de logs
sudo mkdir -p /var/log/datascorched

# Instalar serviço systemd
sudo cp packaging/systemd/datascorched.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable datascorched
sudo systemctl start datascorched
```

## Uso

```bash
# Status do sistema
datascorched status

# Listar sensores
datascorched sensors

# Verificar integridade
datascorched integrity

# Ver eventos de auditoria
datascorched events

# Lockdown manual
datascorched lockdown

# Recuperação
datascorched recover

# Validar configuração
datascorched policy validate

# Verificar integridade dos logs
datascorched audit verify
```

## Configuração

O arquivo de configuração fica em `/etc/datascorched/config.toml`.

Ver `config/config.example.toml` para referência completa.

## Segurança

- Nunca incluir chaves hardcoded
- Nunca gravar segredos em logs
- Proteger chaves em repouso (TPM 2.0 quando disponível)
- Logs append-only com hash chain
- Sandboxing do serviço via systemd
- RBAC para operações privilegiadas

## Licença

MIT

## Autor

Joaquim Pedro de Morais Filho
