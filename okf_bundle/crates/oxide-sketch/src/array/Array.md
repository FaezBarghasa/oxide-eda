---
okf_version: "0.2"
type: Class
title: Array
description: A sketch array — expands to multiple baked primitives at bake time.
resource: crates/oxide-sketch/src/array.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/array/Array
language: rust
---

# Array

A sketch array — expands to multiple baked primitives at bake time.

## Signature

```rust
pub struct Array
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A sketch array — expands to multiple baked primitives at bake time.
Each replica inherits attributes (PadAttr etc.) from `source`, with
per-instance overrides applied by the bake pipeline (number from
`numbering`, position from the array geometry).
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `id`
- `kind`
- `numbering`

## Source
Lines 27–33 in `crates/oxide-sketch/src/array.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [array](/crates/oxide-sketch/src/array.md) |
