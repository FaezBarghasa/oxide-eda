---
okf_version: "0.2"
type: Function
title: spawn
description: Launches the architecture-specific QEMU process in background.
resource: crates/oxide-mcu/src/qemu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:40:58Z"
concept_id: crates/oxide-mcu/src/qemu/spawn
language: rust
---

# spawn

Launches the architecture-specific QEMU process in background.

## Signature

```rust
impl QemuInstance { pub fn spawn(config: QemuConfig) -> Result<Self, QemuError> }
```

## Visibility

- `pub`

## Docstring

Launches the architecture-specific QEMU process in background.

## Source
Lines 51–87 in `crates/oxide-mcu/src/qemu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qemu](/crates/oxide-mcu/src/qemu.md) |
