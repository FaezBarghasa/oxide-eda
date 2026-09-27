---
okf_version: "0.2"
type: Class
title: ErcContext
description: "Normalised, render-independent view of a schematic sheet for ERC purposes."
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/ErcContext
language: rust
---

# ErcContext

Normalised, render-independent view of a schematic sheet for ERC purposes.

## Signature

```rust
pub struct ErcContext
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Normalised, render-independent view of a schematic sheet for ERC purposes.
Built once per run via [`ErcContext::from_snapshot`]; all rules read from it.
[derive(Debug, Clone)]

## Methods

- `paper_size`
- `symbols`
- `wires`
- `buses`
- `labels`
- `junctions`
- `no_connects`
- `bus_entries`
- `child_sheets`
- `nets`
- `children`

## Source
Lines 176–192 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
