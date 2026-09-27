---
okf_version: "0.2"
type: Class
title: XSignal
description: An xSignal represents a complete physical/logical signal path spanning across
resource: crates/oxide-types/src/net.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:59:29Z"
concept_id: crates/oxide-types/src/net/XSignal
language: rust
---

# XSignal

An xSignal represents a complete physical/logical signal path spanning across

## Signature

```rust
pub struct XSignal
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

An xSignal represents a complete physical/logical signal path spanning across
discrete passives (e.g. Driver pin -> Series damping resistor -> DDR4 receiver pin).
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `name`
- `source`
- `destination`
- `intermediate_nets`
- `target_length_microns`
- `tolerance_microns`

## Source
Lines 94–104 in `crates/oxide-types/src/net.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net](/crates/oxide-types/src/net.md) |
