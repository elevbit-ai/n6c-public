# Architecture — SPECTER-NET

## Visão Geral

O SPECTER-NET é um sistema defensivo de monitoramento de espectro RF com resiliência automática de comunicações. A arquitetura é distribuída e baseada em sensores RF que enviam telemetria para um servidor central.

## Componentes

### 1. specter-sensor

Daemon executado próximo ao hardware SDR.

**Responsabilidades:**
- Controlar o SDR via SoapySDR
- Realizar varreduras contínuas do espectro
- Calcular FFT, PSD, noise floor, ocupação
- Detectar anomalias localmente
- Enviar telemetria ao servidor central
- Manter buffer local para operação sem rede

### 2. specter-core

Servidor central que correlaciona dados de múltiplos sensores.

**Responsabilidades:**
- Receber telemetria via API
- Correlacionar eventos entre sensores
- Executar motor de decisão defensivo
- Armazenar eventos e medições
- Disponibilizar API REST
- Comandar mudanças autorizadas nos rádios

### 3. specter-radio-agent

Agente que altera configurações SOMENTE de equipamentos autorizados.

**Responsabilidades:**
- Receber comandos assinados
- Validar allowlist e limites
- Aplicar configuração no rádio
- Confirmar resultado
- Executar rollback em falha

### 4. specter-console

Dashboard web para visualização e operação.

**Páginas:**
- Overview (sensores, rádios, eventos)
- Spectrum (PSD em tempo real)
- Waterfall (histórico temporal)
- RF Events (lista de eventos)
- Managed Radios (estado dos rádios)
- Channel History (histórico de trocas)
- Alerts (alertas ativos)
- Audit (log de auditoria)

## Pipeline DSP

```
SDR IQ Samples
  -> Circular Buffer
  -> Window Function (Hann/Hamming/Blackman)
  -> FFT (Cooley-Tukey radix-2)
  -> Power Spectral Density
  -> Noise Floor Estimation (percentil robusto)
  -> Channel Occupancy Calculation
  -> Anomaly Detection (baseline comparison)
  -> Event Classification
```

## Motor de Detecção

O detector usa múltiplas evidências para classificar eventos:

| Métrica | Peso | Descrição |
|---------|------|-----------|
| Noise Floor Delta | 0.25 | Desvio do piso de ruído basal |
| Occupancy | 0.25 | Ocupação do canal |
| SNR Drop | 0.20 | Queda no SNR |
| Packet Loss | 0.15 | Taxa de perda de pacotes |
| Multi-Sensor | 0.15 | Correlação entre sensores |

**Classificações:**
- NORMAL
- ELEVATED_NOISE
- INTERFERENCE_SUSPECTED
- JAMMING_SUSPECTED
- DEGRADED
- RECOVERING

## Máquina de Estados — Troca de Canal

```
STABLE -> DEGRADING -> CANDIDATE_SELECTED -> CHANGE_PENDING
  -> CHANGING -> VERIFYING -> STABLE_NEW_CHANNEL
                                    |
                              ROLLBACK (em falha)
```

## Segurança

- mTLS entre sensores e servidor
- RBAC em todas as operações
- Comandos com nonce + timestamp (anti-replay)
- Allowlist de canais autorizados
- Rate limiting
- Rollback automático
- Logs de auditoria append-only

## Deploy

```
LAN Privada
  |
  +-- specter-sensor (cada sensor RF)
  |     |
  |     +-- SDR Hardware
  |
  +-- specter-core (servidor central)
  |     |
  |     +-- PostgreSQL
  |     +-- Redis (cache)
  |
  +-- specter-console (dashboard web)
  |
  +-- specter-radio-agent (cada rádio)
        |
        +-- Rádio Autorizado
```

## Confiabilidade

- systemd service com restart automático
- Health checks periódicos
- Reconexão automática
- Buffer local para operação offline
- Graceful shutdown
- Backups do banco
- Rotação de logs
