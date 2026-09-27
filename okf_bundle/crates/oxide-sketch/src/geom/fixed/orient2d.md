---
okf_version: "0.2"
type: Function
title: orient2d
description: "Fixed-point orient2d: sign of the determinant. Returns +1 / -1"
resource: crates/oxide-sketch/src/geom/fixed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/fixed/orient2d
language: rust
---

# orient2d

Fixed-point orient2d: sign of the determinant. Returns +1 / -1

## Signature

```rust
pub fn orient2d(a: FixPoint2, b: FixPoint2, c: FixPoint2) -> i32
```

## Visibility

- `pub`

## Docstring

Fixed-point orient2d: sign of the determinant. Returns +1 / -1
/ 0 — exact arithmetic, no eps tolerance because integers.

## Source
Lines 70–80 in `crates/oxide-sketch/src/geom/fixed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fixed](/crates/oxide-sketch/src/geom/fixed.md) |
