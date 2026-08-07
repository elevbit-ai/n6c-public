# Threat Model

## Asset Protection

- Dados sensíveis em dispositivos móveis e embarcados
- Integridade do sistema operacional
- Confidencialidade de chaves e credenciais
- Disponibilidade do serviço

## Threat Actors

### 1. Atacante com Acesso Físico
- **Capacidade**: Acesso direto ao hardware
- **Técnicas**: Remoção de disco, acesso a portas, manipulação de hardware
- **Defesa**: Sensores de chassis, criptografia de disco, detecção de adulteração

### 2. Dispositivo USB Malicioso
- **Capacidade**: Inserção de dispositivo USB modificado
- **Técnicas**: BadUSB, keylogger, storage malicioso
- **Defesa**: Monitoramento USB, whitelist de dispositivos

### 3. Periférico DMA
- **Capacidade**: Acesso via Thunderbolt/PCIe
- **Técnicas**: DMA attack via Firewire, Thunderbolt
- **Defesa**: Monitoramento PCIe, bloqueio de dispositivos não autorizados

### 4. Boot Externo
- **Capacidade**: Boot de mídia externa
- **Técnicas**: Live USB, boot bypass
- **Defesa**: Secure Boot, verificação de integridade

### 5. Comprometimento Parcial do SO
- **Capacidade**: Acesso como usuário não privilegiado
- **Técnicas**: Escalation de privilégio, persistence
- **Defesa**: Namespaces, cgroups, monitoramento de módulos

## Security Controls

| Control | Type | Implementation |
|---------|------|----------------|
| Secure Boot | Preventive | UEFI verification |
| TPM 2.0 | Detective | PCR validation |
| LUKS2 | Preventive | Disk encryption |
| IMA/EVM | Detective | File integrity |
| dm-verity | Preventive | Block device integrity |
| eBPF | Detective | Runtime monitoring |
| Chassis sensor | Detective | Physical tamper detection |
| USB monitoring | Detective | Device enumeration |
| Audit logging | Detective | Append-only logs with hash chain |
