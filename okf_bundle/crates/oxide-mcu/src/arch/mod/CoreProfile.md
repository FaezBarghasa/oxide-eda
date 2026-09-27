---
okf_version: "0.2"
type: Class
title: CoreProfile
description: Universal MCU Core Profile.
resource: crates/oxide-mcu/src/arch/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:40:05Z"
concept_id: crates/oxide-mcu/src/arch/mod/CoreProfile
language: rust
---

# CoreProfile

Universal MCU Core Profile.

## Signature

```rust
pub struct CoreProfile
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Universal MCU Core Profile.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `name`
- `arch`
- `vendor`
- `bit_width`
- `memory_model`
- `num_cores`
- `max_frequency_hz`
- `qemu_executable`
- `qemu_cpu`
- `qemu_machine`

## Source
Lines 57–68 in `crates/oxide-mcu/src/arch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arch](/crates/oxide-mcu/src/arch/mod.md) |
