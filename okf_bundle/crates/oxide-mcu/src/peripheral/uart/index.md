# uart

## Classs

- [Parity](Parity.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
- [StopBits](StopBits.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
- [UartPeripheral](UartPeripheral.md) — Full UART/USART hardware peripheral model.
- [WordLength](WordLength.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]

## Functions

- [inject_rx_byte](inject_rx_byte.md) — Injects byte received from physical bus / terminal into RX FIFO.
- [inject_rx_byte](inject_rx_byte_1.md) — Injects byte received from physical bus / terminal into RX FIFO.
- [new](new.md)
- [new](new_1.md)
- [pop_tx_byte](pop_tx_byte.md) — Pops byte from TX FIFO to transmit over physical bus.
- [pop_tx_byte](pop_tx_byte_1.md) — Pops byte from TX FIFO to transmit over physical bus.
- [read_byte](read_byte.md) — Reads received byte into MCU firmware.
- [read_byte](read_byte_1.md) — Reads received byte into MCU firmware.
- [write_byte](write_byte.md) — Transmits a byte from MCU firmware.
- [write_byte](write_byte_1.md) — Transmits a byte from MCU firmware.
