---
okf_version: "0.2"
type: Class
title: ErcNet
description: "A logical net: the set of pins and labels connected by wires/junctions."
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/ErcNet
language: rust
---

# ErcNet

A logical net: the set of pins and labels connected by wires/junctions.

## Signature

```rust
pub struct ErcNet
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

A logical net: the set of pins and labels connected by wires/junctions.
Derived during projection via union-find over wire endpoints.
[derive(Debug, Clone)]

## Methods

- `name`
- `class`
- `pin_types`
- `has_driver`
- `has_pullup`

## Source
Lines 152–167 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
