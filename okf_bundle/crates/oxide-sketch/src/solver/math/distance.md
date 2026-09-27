---
okf_version: "0.2"
type: Function
title: distance
description: Euclidean distance between two points.
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/distance
language: rust
---

# distance

Euclidean distance between two points.

## Signature

```rust
pub fn distance(a: Vec2, b: Vec2) -> f64
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Euclidean distance between two points.
[inline]

## Source
Lines 86–88 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| calls | [norm](/crates/oxide-sketch/src/solver/math/norm.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
