---
okf_version: "0.2"
type: Function
title: write_from_terminal
description: Queues user input string to be read by MCU UART receiver.
resource: crates/oxide-mcu/src/uart.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:15Z"
concept_id: crates/oxide-mcu/src/uart/write_from_terminal_1
language: rust
---

# write_from_terminal

Queues user input string to be read by MCU UART receiver.

## Signature

```rust
pub fn write_from_terminal(&mut self, text: &str)
```

## Visibility

- `pub`

## Docstring

Queues user input string to be read by MCU UART receiver.

## Source
Lines 39–43 in `crates/oxide-mcu/src/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/uart.md) |
