---
okf_version: "0.2"
type: Class
title: InterruptLine
description: State for a single interrupt line.
resource: crates/oxide-mcu/src/peripheral/nvic.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:15:35Z"
concept_id: crates/oxide-mcu/src/peripheral/nvic/InterruptLine
language: rust
---

# InterruptLine

State for a single interrupt line.

## Signature

```rust
pub struct InterruptLine
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

State for a single interrupt line.
[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]

## Methods

- `enabled`
- `pending`
- `active`
- `priority`

## Source
Lines 9–14 in `crates/oxide-mcu/src/peripheral/nvic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nvic](/crates/oxide-mcu/src/peripheral/nvic.md) |
