---
okf_version: "0.2"
type: Class
title: DecapModel
description: Decoupling Capacitor Equivalent Circuit Model (RLC).
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn/DecapModel
language: rust
---

# DecapModel

Decoupling Capacitor Equivalent Circuit Model (RLC).

## Signature

```rust
pub struct DecapModel
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Decoupling Capacitor Equivalent Circuit Model (RLC).
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `capacitance_f`
- `esr_ohms`
- `esl_henries`
- `via_inductance_henries`
- `count`

## Source
Lines 39–45 in `crates/oxide-rf/src/pdn.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdn](/crates/oxide-rf/src/pdn.md) |
