# Security Policy — SPECTER-NET

**N6 Cybernetics**

## Princípios de Design

| Princípio | Descrição |
|-----------|-----------|
| Defesa em Profundidade | Múltiplas camadas de segurança |
| Menor Privilégio | Serviços operam com privilégios mínimos |
| Fail-Safe | Falhas preservam dados e estado |
| Auditoria Total | Todos os eventos são registrados |
| Recuperação | Sempre permitir restauração administrativa |

## Modelo de Ameaças

### Vetores Considerados

| Ameaça | Vetor | Defesa |
|--------|-------|--------|
| Jamming | Interferência RF deliberada | Detecção multi-evidência, troca de canal |
| Spoofing RF | Falsificação de sinais | Validação de sensores, mTLS |
| Acesso não autorizado | APIs expostas | RBAC, TLS, rate limiting |
| Replay | Reenvio de comandos | Nonce + timestamp por comando |
| Configuração indevida | Canais não autorizados | Allowlist, validação em 8 etapas |

### Limitações Documentadas

O sistema não garante proteção contra:
- Jamming de banda larga extremo
- Comprometimento físico do hardware SDR
- Ataques ao canal de comunicação sensores-servidor
- Falhas de hardware não detectáveis

## Controle de Acesso

### Perfis

| Perfil | Permissões |
|--------|------------|
| VIEWER | Leitura apenas |
| OPERATOR | Leitura + comandos operacionais |
| ADMIN | Configuração + comandos administrativos |
| AUDITOR | Leitura + logs de auditoria |

### Comandos Administrativos

Todos os comandos de mudança de canal requerem:
1. Autenticação mTLS
2. Autorização RBAC
3. Validação de allowlist
4. Nonce único
5. Timestamp válido
6. Rate limiting
7. Confirmação
8. Auditoria

## Criptografia

| Algoritmo | Uso |
|-----------|-----|
| TLS 1.3 | Transporte |
| mTLS | Autenticação sensor-servidor |
| HMAC-SHA256 | Assinatura de comandos |

## Auditoria

Todos os eventos são registrados:
- Timestamp
- Usuário/sensor
- Ação realizada
- Resultado
- IP de origem

Logs são append-only com hash chain para verificação de integridade.
