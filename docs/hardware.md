# Hardware Requirements

## Servidor

- TPM 2.0
- Secure Boot/UEFI
- SSD com criptografia (LUKS2)
- Sensor de intrusão de gabinete (se disponível)
- NIC gerenciável
- BMC com configuração segura (quando usado)

## Embarcados

- TPM ou Secure Element
- Sensor físico de gabinete
- GPIO
- Watchdog
- Armazenamento criptografado
- Boot verificado (dm-verity)

## Sensores Suportados

| Sensor | Tipo | Plataforma |
|--------|------|------------|
| Temperatura | hwmon/thermal | Linux |
| USB | /sys/bus/usb/devices | Linux |
| PCIe | lspci / sysfs | Linux |
| TPM | /dev/tpm0 | Linux |
| Secure Boot | efi/efivars | Linux UEFI |
| Chassis | IPMI/sysfs | Variável |

## Requisitos do Sistema Operacional

- Linux (kernel 5.4+)
- systemd
- UEFI (para Secure Boot)
- TPM 2.0 driver (para TPM)
- hwmon subsystem (para temperatura)
