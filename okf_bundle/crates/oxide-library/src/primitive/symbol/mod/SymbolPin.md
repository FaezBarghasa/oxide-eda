---
okf_version: "0.2"
type: Class
title: SymbolPin
description: One symbol pin.
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/SymbolPin
language: rust
---

# SymbolPin

One symbol pin.

## Signature

```rust
pub struct SymbolPin
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One symbol pin.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `number`
- `name`
- `electrical`
- `position`
- `orientation`
- `length`
- `description`
- `function`
- `pin_package_length`
- `propagation_delay_ns`
- `designator_visible`
- `name_visible`
- `inside_symbol`
- `inside_edge_symbol`
- `outside_edge_symbol`
- `outside_symbol`
- `hidden`
- `locked`
- `part_number`

## Source
Lines 89–160 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
