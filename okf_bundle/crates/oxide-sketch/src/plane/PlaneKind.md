---
okf_version: "0.2"
type: Class
title: PlaneKind
description: "[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-sketch/src/plane.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/plane/PlaneKind
language: rust
---

# PlaneKind

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub enum PlaneKind
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

- `offset_z_expr`

## Source
Lines 29–41 in `crates/oxide-sketch/src/plane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [plane](/crates/oxide-sketch/src/plane.md) |
