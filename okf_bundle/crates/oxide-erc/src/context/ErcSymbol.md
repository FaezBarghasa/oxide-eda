---
okf_version: "0.2"
type: Class
title: ErcSymbol
description: "A placed component instance. Its `pins` are already transformed to"
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/ErcSymbol
language: rust
---

# ErcSymbol

A placed component instance. Its `pins` are already transformed to

## Signature

```rust
pub struct ErcSymbol
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

A placed component instance. Its `pins` are already transformed to
world-space coordinates (rotation + mirror applied during projection).
[derive(Debug, Clone)]

## Methods

- `uuid`
- `reference`
- `value`
- `position`
- `is_power`
- `pins`
- `attrs`

## Source
Lines 76–87 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
