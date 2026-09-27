---
okf_version: "0.2"
type: Function
title: dot
description: "Dot product `a · b = a.x·b.x + a.y·b.y`."
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/dot
language: rust
---

# dot

Dot product `a · b = a.x·b.x + a.y·b.y`.

## Signature

```rust
pub fn dot(a: Vec2, b: Vec2) -> f64
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Dot product `a · b = a.x·b.x + a.y·b.y`.
[inline]

## Source
Lines 57–59 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [symmetric_about_line](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_line.md) |
