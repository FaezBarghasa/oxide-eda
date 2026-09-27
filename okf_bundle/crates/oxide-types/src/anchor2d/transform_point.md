---
okf_version: "0.2"
type: Function
title: transform_point
description: Transform a local-space point to world space using this transform.
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/transform_point
language: rust
---

# transform_point

Transform a local-space point to world space using this transform.

## Signature

```rust
impl Transform2D { pub fn transform_point(&self, local: Vec2d) -> Vec2d }
```

## Visibility

- `pub`

## Docstring

Transform a local-space point to world space using this transform.
[must_use]

## Source
Lines 154–160 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
