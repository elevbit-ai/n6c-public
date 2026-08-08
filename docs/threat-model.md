# Threat Model — SPECTER-NET

## Atores Considerados

| Ator | Capacidade | Vetor |
|------|-----------|-------|
| Jammer externo | Transmissor RF | Interferência de banda larga/estreita |
| Intruso de rede | Acesso LAN | Interceptação, injeção |
| Insider malicioso | Credenciais válidas | Abuso de API, configuração indevida |
| Atacante físico | Acesso ao hardware | Manipulação de SDR, cables |

## Defesas

### Contra Jamming

- Detecção multi-evidência (noise floor, occupancy, SNR, correlação)
- Troca automática de canal para frequência limpa
- Allowlist de canais autorizados
- Cooldown para evitar oscilação
- Rollback em falha

### Contra Intrusão de Rede

- TLS 1.3 obrigatório
- mTLS entre sensores e servidor
- Rate limiting em todas as APIs
- Validação de input
- RBAC em todas as operações

### Contra Insider

- Logs de auditoria append-only
- Separação de perfis (VIEWER/OPERATOR/ADMIN/AUDITOR)
- Comandos assinados com nonce + timestamp
- Validação de allowlist em múltiplas etapas
- Rollback automático

### Contra Acesso Físico

- Sensores de chassis (quando disponível)
- Criptografia de dados em repouso
- Validação de integridade do hardware
- Alertas de desconexão

## Limitações

O sistema NÃO protege contra:
- Jamming de banda larga extremo que cubra toda a faixa
- Comprometimento físico prolongado do hardware
- Vulnerabilidades de implementação de hardware
- Chaves comprometidas antes da instalação
- Ataques anteriores à implantação do sistema
