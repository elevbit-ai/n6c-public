# Architecture

## Overview

Data-Scorched Safe é composto por 8 crates Rust organizados em um workspace:

```
datascorched/
├── Cargo.toml (workspace)
├── crates/
│   ├── common/      - Tipos compartilhados, configuração, erros
│   ├── hardware/    - Monitoramento de hardware
│   ├── integrity/   - Verificação de integridade
│   ├── policy/      - Engine de política e risco
│   ├── crypto/      - Operações criptográficas
│   ├── response/    - Engine de resposta
│   ├── audit/       - Logging de auditoria
│   └── agent/       - Daemon principal e CLI
```

## Fluxo de Dados

```
Hardware Sensors → hardware-monitor → Integrity Check → Policy Engine → Response Engine
                                                      ↓
                                               Audit Logger
```

## States

```
NORMAL → ELEVATED → LOCKDOWN → QUARANTINE → RECOVERY → NORMAL
```

## Dependency Graph

```
agent → policy, response, audit, crypto, hardware, integrity, common
response → policy, common
policy → common
audit → crypto, common
crypto → common
integrity → hardware, common
hardware → common
common → (no dependencies)
```
