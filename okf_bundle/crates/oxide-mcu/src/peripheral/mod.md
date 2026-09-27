---
okf_version: "0.2"
type: Module
title: peripheral
description: Peripheral Emulation Subsystem for Virtual ARM MCUs.
resource: crates/oxide-mcu/src/peripheral/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:59Z"
concept_id: crates/oxide-mcu/src/peripheral/mod
language: rust
---

# peripheral

Peripheral Emulation Subsystem for Virtual ARM MCUs.

## Docstring

Peripheral Emulation Subsystem for Virtual ARM MCUs.

Provides models for:
- [`nvic::Nvic`]: Nested Vectored Interrupt Controller (priority grouping & dispatch)
- [`systick::SysTick`]: 24-bit RTOS tick timer
- [`gpio::GpioPort`]: Multi-port GPIO (A..K with MODER, OTYPER, PUPDR, BSRR)
- [`timer::TimerPeripheral`]: Basic, General-Purpose, and Advanced Timers with PWM & dead-time
- [`adc::AdcPeripheral`]: Multi-channel 8..16-bit SAR ADC with SPICE net voltage sampling
- [`dac::DacPeripheral`]: Dual 12-bit DAC with waveform generators
- [`comparator::ComparatorPeripheral`]: Fast analog comparators with hysteresis
- [`uart::UartPeripheral`]: USART/UART/LPUART with fractional baud & FIFOs
- [`spi::SpiPeripheral`]: SPI master/slave & QSPI/OSPI flash controllers
- [`i2c::I2cPeripheral`]: I2C/SMBus master/slave with clock stretching
- [`dma::DmaController`]: Multi-stream DMA with circular buffer mode
- [`can::CanPeripheral`]: CAN 2.0A/B & CAN-FD controllers with mailboxes
- [`ethernet::EthernetMacPeripheral`]: 10/100M Ethernet MAC with DMA ring
- [`usb::UsbDevicePeripheral`]: USB Device controller with EP0..EP8
- [`watchdog::WatchdogPeripheral`]: Independent (IWDG) & Window (WWDG) watchdogs
- [`rtc::RtcPeripheral`]: Real-Time Clock with BCD calendar & alarms
- [`crc::CrcPeripheral`]: Hardware CRC-32/16/8 computation engine
- [`rng::RngPeripheral`]: Hardware True / Pseudo Random Number Generator
