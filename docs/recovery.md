# Recovery — SPECTER-NET

## Recuperação de Falhas

### Sensor Offline

1. Verificar conexão física do SDR
2. Reiniciar serviço: `sudo systemctl restart specter-sensor`
3. Verificar logs: `journalctl -u specter-sensor`
4. Se persistir, reconectar USB e reiniciar

### Servidor Core Offline

1. Verificar PostgreSQL: `sudo systemctl status postgresql`
2. Verificar conectividade da rede
3. Reiniciar: `sudo systemctl restart specter-core`
4. Verificar logs: `journalctl -u specter-core`

### Falha na Troca de Canal

O sistema executa rollback automaticamente:
1. Detecta falha na verificação pós-troca
2. Reverte para o canal anterior
3. Registra evento de rollback
4. Mantém o canal anterior até nova detecção

### Banco de Dados

```bash
# Restaurar backup
psql -U specter -d specter_net < backup_YYYYMMDD.sql

# Verificar integridade
psql -U specter -d specter_net -c "SELECT count(*) FROM rf_events;"
```

## Rollback Manual

```bash
# Via API
curl -X POST http://localhost:8080/api/v1/radios/{id}/channel-change \
  -H "Content-Type: application/json" \
  -d '{"target_channel": 1, "reason": "Manual rollback"}'
```

## Limpeza de Dados

```bash
# Remover medições antigas (> 30 dias)
psql -U specter -d specter_net -c "
  DELETE FROM spectrum_measurements
  WHERE timestamp < NOW() - INTERVAL '30 days';
"

# Remover eventos antigos (> 90 dias)
psql -U specter -d specter_net -c "
  DELETE FROM rf_events
  WHERE timestamp < NOW() - INTERVAL '90 days';
"
```
