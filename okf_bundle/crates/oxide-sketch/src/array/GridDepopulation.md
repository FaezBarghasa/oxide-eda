---
okf_version: "0.2"
type: Class
title: GridDepopulation
description: "Predicate evaluated per `(i, j)` index in a Grid array."
resource: crates/oxide-sketch/src/array.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/array/GridDepopulation
language: rust
---

# GridDepopulation

Predicate evaluated per `(i, j)` index in a Grid array.

## Signature

```rust
pub struct GridDepopulation
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Predicate evaluated per `(i, j)` index in a Grid array.
`true` keeps the instance; `false` skips it. `i`, `j`, `nx`, `ny`
are bound in scope.

v0.23 — `suppressed_instances` is an explicit list of `(i, j)`
indices to skip *in addition to* whatever `mask_expr` evaluates to.
The bake skips an instance when EITHER predicate fires (mask returns
false OR the index is in the list). This lets the Properties
panel's per-instance checkbox grid (Phase B5 UI) toggle individual
pads without needing to round-trip through an expression parser.
Polar arrays use `j = 0` for every entry.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `mask_expr`
- `suppressed_instances`

## Source
Lines 87–91 in `crates/oxide-sketch/src/array.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [array](/crates/oxide-sketch/src/array.md) |
