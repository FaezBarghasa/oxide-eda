---
okf_version: "0.2"
type: Function
title: all
description: Iterate the canonical fixed-set layers in stable display order.
resource: crates/oxide-types/src/layer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/layer/all
language: rust
---

# all

Iterate the canonical fixed-set layers in stable display order.

## Signature

```rust
impl OxideLayer { pub fn all() -> impl Iterator<Item = OxideLayer> }
```

## Visibility

- `pub`

## Docstring

Iterate the canonical fixed-set layers in stable display order.
Excludes the parameterised variants (`InnerCopper`, `Mechanical`,
`User`); callers iterating those provide their own indices.

## Source
Lines 112–130 in `crates/oxide-types/src/layer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layer](/crates/oxide-types/src/layer.md) |
