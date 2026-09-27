---
okf_version: "0.2"
type: Function
title: add_bus
description: Adds a multi-bit bus to the harness.
resource: crates/oxide-net/src/harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:32:38Z"
concept_id: crates/oxide-net/src/harness/add_bus_1
language: rust
---

# add_bus

Adds a multi-bit bus to the harness.

## Signature

```rust
pub fn add_bus(&mut self, bus_name: &str, msb: u32, lsb: u32)
```

## Visibility

- `pub`

## Docstring

Adds a multi-bit bus to the harness.

## Source
Lines 49–55 in `crates/oxide-net/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/oxide-net/src/harness.md) |
