---
okf_version: "0.2"
type: Function
title: inspect
description: "Parses ELF header, Intel HEX, or raw binary."
resource: crates/oxide-mcu/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:44:25Z"
concept_id: crates/oxide-mcu/src/firmware/inspect_1
language: rust
---

# inspect

Parses ELF header, Intel HEX, or raw binary.

## Signature

```rust
pub fn inspect(path: impl AsRef<Path>, target: McuTarget) -> Result<Self, FirmwareError>
```

## Visibility

- `pub`

## Docstring

Parses ELF header, Intel HEX, or raw binary.

## Source
Lines 557–608 in `crates/oxide-mcu/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/oxide-mcu/src/firmware.md) |
