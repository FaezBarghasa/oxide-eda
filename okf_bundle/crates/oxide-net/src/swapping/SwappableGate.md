---
okf_version: "0.2"
type: Class
title: SwappableGate
description: "Swappable Part Gate (e.g., Gate A, B, C, D in a quad NAND IC)."
resource: crates/oxide-net/src/swapping.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:13:08Z"
concept_id: crates/oxide-net/src/swapping/SwappableGate
language: rust
---

# SwappableGate

Swappable Part Gate (e.g., Gate A, B, C, D in a quad NAND IC).

## Signature

```rust
pub struct SwappableGate
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Swappable Part Gate (e.g., Gate A, B, C, D in a quad NAND IC).
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `gate_id`
- `swap_group_id`
- `pin_mappings`

## Source
Lines 23–27 in `crates/oxide-net/src/swapping.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [swapping](/crates/oxide-net/src/swapping.md) |
