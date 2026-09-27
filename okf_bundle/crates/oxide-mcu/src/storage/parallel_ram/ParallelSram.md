---
okf_version: "0.2"
type: Class
title: ParallelSram
description: "Asynchronous Parallel SRAM (e.g. ISSI IS62WV51216, Cypress CY62167EV30 16-bit SRAM)."
resource: crates/oxide-mcu/src/storage/parallel_ram.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:29Z"
concept_id: crates/oxide-mcu/src/storage/parallel_ram/ParallelSram
language: rust
---

# ParallelSram

Asynchronous Parallel SRAM (e.g. ISSI IS62WV51216, Cypress CY62167EV30 16-bit SRAM).

## Signature

```rust
pub struct ParallelSram
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Asynchronous Parallel SRAM (e.g. ISSI IS62WV51216, Cypress CY62167EV30 16-bit SRAM).
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `base_address`
- `size_bytes`
- `bus_width_bits`
- `memory`

## Source
Lines 7–13 in `crates/oxide-mcu/src/storage/parallel_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_ram](/crates/oxide-mcu/src/storage/parallel_ram.md) |
