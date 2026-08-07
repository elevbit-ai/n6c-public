# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in Data-Scorched Safe, please report it responsibly.

## Security Design Principles

1. **Defense in Depth**: Múltiplas camadas de segurança
2. **Least Privilege**: Serviço roda com privilégios mínimos necessários
3. **Fail-Safe**: Falhas não corrompem dados nem apagam evidências
4. **No Destructive Actions**: Nunca executar destruição automática de dados
5. **Audit Everything**: Todos os eventos são registrados
6. **Recovery Required**: Sempre permitir recuperação administrativa

## Threat Model

### Atacantes Considerados
- Acesso físico ao dispositivo
- Dispositivo USB malicioso
- Periférico DMA
- Boot externo
- Comprometimento parcial do SO
- Insider com conta não privilegiada

### Limitações Documentadas
O sistema NÃO protege contra:
- Atacante com acesso físico prolongado e equipamento de laboratório
- Comprometimento de firmware não detectável
- Bugs no processador
- Chaves já expostas
- Ataques anteriores à ativação das proteções

## Secure Development

- `cargo fmt` obrigatório
- `cargo clippy` sem warnings
- `cargo test` todos passando
- `cargo audit` sem vulnerabilidades conhecidas
- Dependências versionadas no lockfile
- Builds reproduzíveis quando possível
