---
okf_version: "0.2"
type: Function
title: kill
description: Terminates the running QEMU instance.
resource: crates/oxide-mcu/src/qemu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:40:58Z"
concept_id: crates/oxide-mcu/src/qemu/kill
language: rust
---

# kill

Terminates the running QEMU instance.

## Signature

```rust
impl QemuInstance { pub fn kill(&mut self) -> Result<(), std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Terminates the running QEMU instance.

## Source
Lines 90–95 in `crates/oxide-mcu/src/qemu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qemu](/crates/oxide-mcu/src/qemu.md) |
