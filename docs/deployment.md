# Deployment

## Quick Start

```bash
# 1. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Compilar
cargo build --release

# 3. Instalar
sudo cp target/release/datascorched /usr/local/bin/
sudo mkdir -p /etc/datascorched /var/log/datascorched
sudo cp config/config.example.toml /etc/datascorched/config.toml

# 4. Configurar
sudo nano /etc/datascorched/config.toml

# 5. Instalar serviço
sudo cp packaging/systemd/datascorched.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable datascorched
sudo systemctl start datascorched

# 6. Verificar
datascorched status
datascorched sensors
datascorched integrity
```

## Production Deployment

1. Habilitar Secure Boot no UEFI
2. Provisionar TPM 2.0
3. Configurar LUKS2 para disco
4. Ajustar limiares de risco na configuração
5. Habilitar remote logging (opcional)
6. Configurar alertas
7. Testar cenários de lockdown e recovery

## Monitoring

```bash
# Status em tempo real
watch -n 5 datascorched status

# Logs do systemd
journalctl -u datascorched -f

# Events
datascorched events
```
