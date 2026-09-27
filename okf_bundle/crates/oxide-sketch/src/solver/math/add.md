---
okf_version: "0.2"
type: Function
title: add
description: "Component-wise addition `a + b`."
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/add
language: rust
---

# add

Component-wise addition `a + b`.

## Signature

```rust
pub fn add(a: Vec2, b: Vec2) -> Vec2
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Component-wise addition `a + b`.
[inline]

## Source
Lines 45–47 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [midpoint](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/midpoint.md) |
| called_by | [symmetric_about_line](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_line.md) |
| called_by | [symmetric_about_point](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_point.md) |
