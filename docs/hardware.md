# Hardware Support — SPECTER-NET

## SDR Suportados

### RTL-SDR

- **Tipo:** Receptor passivo
- **Custo:** Baixo
- **Faixa:** 24 MHz — 1.766 GHz
- **Uso recomendado:** Protótipos, sensores passivos

### USRP

- **Tipo:** Transceptor profissional
- **Custo:** Alto
- **Faixa:** Variável conforme modelo
- **Uso recomendado:** Aplicações profissionais, melhor sincronização

### SoapySDR

- **Tipo:** Camada de abstração
- **Suporte:** Qualquer dispositivo com driver SoapySDR
- **Uso recomendado:** Flexibilidade de hardware

## Configuração do Sensor

Cada sensor deve possuir:
- Identificador único (UUID)
- Localização lógica configurável
- Relógio sincronizado por NTP/PTP
- SDR conectado
- Processo `specter-sensor` em execução
- Conexão autenticada com o servidor

## Parâmetros RF

| Parâmetro | Padrão | Descrição |
|-----------|--------|-----------|
| sample_rate | 2.4 MHz | Taxa de amostragem |
| center_frequency | 433 MHz | Frequência central |
| bandwidth | 2 MHz | Largura de banda |
| gain | 30 dB | Ganho do receptor |
| fft_size | 4096 | Tamanho da FFT |
| window | Hann | Função de janela |
| overlap | 0.5 | Sobreposição |
| dwell_time | 100 ms | Tempo de permanência |

## Tratamento de Erros

O sensor trata:
- Overflow de buffer
- Perda do SDR
- Driver travado
- Ganho saturado
- Relógio inconsistente
- Desconexão USB/rede

## SDR Não Detectado

Se nenhum SDR estiver conectado, o programa exibe:

```
NO RF DEVICE AVAILABLE
```

O modo de produção não gera espectro fictício.
