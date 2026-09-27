---
okf_version: "0.2"
type: Class
title: EcoAction
description: Atomic ECO action to synchronize schematic and PCB.
resource: crates/oxide-net/src/eco.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:13:59Z"
concept_id: crates/oxide-net/src/eco/EcoAction
language: rust
---

# EcoAction

Atomic ECO action to synchronize schematic and PCB.

## Signature

```rust
pub enum EcoAction
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Atomic ECO action to synchronize schematic and PCB.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `reference`
- `value`
- `footprint_id`
- `suggested_pos`
- `reference`
- `uuid`
- `reference`
- `old_value`
- `new_value`
- `old_footprint_id`
- `new_footprint_id`
- `name`
- `net_id`
- `name`
- `net_id`
- `reference`
- `pad_number`
- `old_net`
- `new_net`
- `new_net_id`
- `old_reference`
- `new_reference`
- `component_reference`
- `pin_a`
- `pin_b`
- `net_a`
- `net_b`

## Source
Lines 15–67 in `crates/oxide-net/src/eco.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eco](/crates/oxide-net/src/eco.md) |
