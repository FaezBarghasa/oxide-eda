---
okf_version: "0.2"
type: Class
title: ArrayKind
description: "[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-sketch/src/array.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/array/ArrayKind
language: rust
---

# ArrayKind

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub enum ArrayKind
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "PascalCase")]

## Methods

- `source`
- `count_expr`
- `dx_expr`
- `dy_expr`
- `source`
- `nx_expr`
- `ny_expr`
- `dx_expr`
- `dy_expr`
- `depopulation`
- `source`
- `center`
- `count_expr`
- `sweep_angle_expr`
- `depopulation`

## Source
Lines 37–73 in `crates/oxide-sketch/src/array.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [array](/crates/oxide-sketch/src/array.md) |
