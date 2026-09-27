---
okf_version: "0.2"
type: Class
title: QemuConfig
description: Configuration for starting virtual MCU target in QEMU.
resource: crates/oxide-mcu/src/qemu.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:40:58Z"
concept_id: crates/oxide-mcu/src/qemu/QemuConfig
language: rust
---

# QemuConfig

Configuration for starting virtual MCU target in QEMU.

## Signature

```rust
pub struct QemuConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Configuration for starting virtual MCU target in QEMU.
[derive(Debug, Clone)]

## Methods

- `qemu_bin`
- `firmware`
- `gdb_port`
- `serial_socket_path`
- `enable_semihosting`

## Source
Lines 22–28 in `crates/oxide-mcu/src/qemu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qemu](/crates/oxide-mcu/src/qemu.md) |
