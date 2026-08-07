# Security Policy — Data-Scorched Safe

**N6 Cybernetics**

## Reportar Vulnerabilidades

Se você descobrir uma vulnerabilidade de segurança no Data-Scorched Safe, por favor reporte de forma responsável.

## Princípios de Design

| Princípio | Descrição |
|-----------|-----------|
| Defesa em Profundidade | Múltiplas camadas de segurança sobrepostas |
| Menor Privilégio | Serviço opera com privilégios mínimos necessários |
| Fail-Safe | Falhas preservam dados e evidências |
| Sem Destruição | Nunca executar destruição automática de dados |
| Auditoria Total | Todos os eventos são registrados |
| Recuperação | Sempre permitir restauração administrativa |

## Modelo de Ameaças

### Atores Considerados

| Ameaça | Vetor | Defesa |
|--------|-------|--------|
| Acesso físico | Remoção de disco, manipulação de hardware | Sensores de chassis, criptografia LUKS2 |
| USB malicioso | BadUSB, keylogger, storage modificado | Monitoramento USB, whitelist |
| Periférico DMA | Thunderbolt, Firewire | Monitoramento PCIe, bloqueio |
| Boot externo | Live USB, boot bypass | Secure Boot, verificação de integridade |
| Comprometimento parcial | Escalation de privilégio | Namespaces, cgroups, eBPF |
| Insider | Conta não privilegiada | RBAC, logs de auditoria |

### Limitações Documentadas

O sistema não garante proteção contra:

- Atacantes com acesso físico prolongado e equipamento de laboratório
- Comprometimento de firmware não detectável por sensores disponíveis
- Vulnerabilidades de implementação de hardware (side-channel)
- Chaves criptográficas já expostas antes da ativação do sistema
- Ataques realizados antes da instalação e configuração do Data-Scorched Safe

## Desenvolvimento Seguro

| Ferramenta | Finalidade |
|------------|------------|
| `cargo fmt` | Formatação padronizada |
| `cargo clippy` | Análise estática de código |
| `cargo test` | Testes unitários e de integração |
| `cargo audit` | Verificação de vulnerabilidades em dependências |
| Lockfile | Versões fixadas de todas as dependências |
| Hash chain | Integridade verificável dos logs de auditoria |

## Criptografia

| Algoritmo | Uso |
|-----------|-----|
| AES-256-GCM | Criptografia de dados em repouso |
| HMAC-SHA256 | Assinatura e verificação de integridade |
| SHA-256 | Hash chain nos logs de auditoria |
| Derivação de chaves | Separação de chaves por finalidade |

---

*N6 Cybernetics — Hardened Security Systems*
