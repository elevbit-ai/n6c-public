# Recovery Procedures

## Recovery Mode

O modo RECOVERY é acessível apenas por administrador autorizado.

## Procedimento de Recovery

### 1. Verificar Estado
```bash
datascorched status
```

### 2. Iniciar Recovery
```bash
datascorched recover
```

### 3. Autenticação Administrativa
- MFA requerido (quando configurado)
- Operador deve ter credenciais válidas
- Recovery é registrado em audit log

### 4. Restaurar Estado
```bash
# Verificar integridade
datascorched integrity

# Validar configuração
datascorched policy validate

# Reiniciar serviço
sudo systemctl restart datascorched

# Verificar status
datascorched status
```

## Recovery após Lockdown

1. Acessar console físico ou SSH com chave autorizada
2. Executar `datascorched recover`
3. Fornecer credenciais de administrador
4. Sistema transita de LOCKDOWN → RECOVERY → NORMAL
5. Verificar logs para investigação

## Recovery após Quarantine

1. Todas as etapas do lockdown
2. Reativar serviços não essenciais manualmente
3. Reconectar interfaces de rede
4. Verificar integridade completa

## Troubleshooting

### Serviço não inicia
```bash
# Verificar logs
journalctl -u datascorched -n 50

# Verificar configuração
datascorched policy validate

# Verificar permissões
ls -la /etc/datascorched/
ls -la /var/log/datascorched/
```

### Sensores indisponíveis
```bash
# Verificar sensores
datascorched sensors

# Verificar hardware
ls /sys/class/hwmon/
ls /sys/bus/usb/devices/
ls /dev/tpm*
```

### Logs corrompidos
```bash
# Verificar integridade
datascorched audit verify

# Se inválido, investigar causa
# Recursos de recuperação devem ser consultados
```

## Backup e Restauração

### Backup de Configuração
```bash
sudo cp /etc/datascorched/config.toml /backup/config.toml.bak
```

### Backup de Logs
```bash
sudo cp -r /var/log/datascorched/ /backup/audit-logs/
```

### Restauração
```bash
sudo cp /backup/config.toml.bak /etc/datascorched/config.toml
sudo systemctl restart datascorched
```
