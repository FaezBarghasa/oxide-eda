---
okf_version: "0.2"
type: Module
title: lib
description: "`oxide-mcu` — Virtual ARM Microcontroller & Comprehensive Peripheral Co-Simulation Engine."
resource: crates/oxide-mcu/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:50:22Z"
concept_id: crates/oxide-mcu/src/lib
language: rust
---

# lib

`oxide-mcu` — Virtual ARM Microcontroller & Comprehensive Peripheral Co-Simulation Engine.

## Docstring

`oxide-mcu` — Virtual ARM Microcontroller & Comprehensive Peripheral Co-Simulation Engine.

Provides:
- [`FirmwareImage`], [`ArmArch`], [`ArmCore`], & [`McuFamily`]: Architecture profiling, core mapping, and SoC family presets.
- [`peripheral`]: Complete hardware peripheral emulation suite (NVIC, SysTick, GPIO, Timers/PWM, ADC, DAC, COMP, UART, SPI, I2C, DMA, CAN, Ethernet, USB, Watchdogs, RTC, CRC, RNG).
- [`PinBridge`], [`VirtualPinState`], & [`LogicLevel`]: Real-time synchronization between virtual MCU pins and SPICE schematic nets.
- [`QemuConfig`] & [`QemuInstance`]: Managed QEMU supervisor and GDB remote debugging stub.
- [`VirtualUart`]: Bi-directional UART serial terminal buffer.

## Relationships

| Type | Target |
|------|--------|
| related | [test_arm_core_and_architecture_mapping](/crates/oxide-mcu/src/lib/test_arm_core_and_architecture_mapping.md) |
| related | [test_multi_vendor_mcu_targets](/crates/oxide-mcu/src/lib/test_multi_vendor_mcu_targets.md) |
| related | [test_peripheral_suite](/crates/oxide-mcu/src/lib/test_peripheral_suite.md) |
| related | [test_pin_bridge_logic_and_adc](/crates/oxide-mcu/src/lib/test_pin_bridge_logic_and_adc.md) |
| related | [test_external_memory_suite](/crates/oxide-mcu/src/lib/test_external_memory_suite.md) |
| related | [test_display_and_touch_simulation](/crates/oxide-mcu/src/lib/test_display_and_touch_simulation.md) |
