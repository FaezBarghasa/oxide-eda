---
okf_version: "0.2"
type: Class
title: Transform2D
description: 2D transform with an explicit pivot/anchor point stored in world space.
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/Transform2D
language: rust
---

# Transform2D

2D transform with an explicit pivot/anchor point stored in world space.

## Signature

```rust
pub struct Transform2D
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

2D transform with an explicit pivot/anchor point stored in world space.

See the [module docs](self) for the full coordinate model.
[derive(Debug, Clone, PartialEq)]

## Methods

- `pivot_world`
- `local_offset`
- `size`
- `rotation_rad`

## Source
Lines 34–43 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
