---
okf_version: "0.2"
type: Class
title: PadShape
description: Pad copper outline.
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/PadShape
language: rust
---

# PadShape

Pad copper outline.

## Signature

```rust
pub enum PadShape
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`
- `serde(tag = "shape", rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

Pad copper outline.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
[serde(tag = "shape", rename_all = "PascalCase")]

## Methods

- `radius_ratio_expr`
- `chamfer_ratio_expr`
- `corners`

## Source
Lines 335–351 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
