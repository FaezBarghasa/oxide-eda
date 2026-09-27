---
okf_version: "0.2"
type: Function
title: translate
description: "Translate the entire transform: both `pivot_world` and the derived"
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/translate
language: rust
---

# translate

Translate the entire transform: both `pivot_world` and the derived

## Signature

```rust
impl Transform2D { pub fn translate(&mut self, delta: Vec2d) }
```

## Visibility

- `pub`

## Docstring

Translate the entire transform: both `pivot_world` and the derived
`origin_world` shift by `delta`.

## Source
Lines 171–174 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
