---
okf_version: "0.2"
type: Function
title: orient2d
description: "Orientation of triangle `(a, b, c)`. Positive = CCW (left turn at"
resource: crates/oxide-sketch/src/geom/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/predicates/orient2d
language: rust
---

# orient2d

Orientation of triangle `(a, b, c)`. Positive = CCW (left turn at

## Signature

```rust
pub fn orient2d(a: Point2, b: Point2, c: Point2) -> Sign
```

## Visibility

- `pub`

## Docstring

Orientation of triangle `(a, b, c)`. Positive = CCW (left turn at
b), Negative = CW (right turn), Zero = colinear within tolerance.

Determinant form:
```text
det = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
```
equals twice the signed area of the triangle. Positive when the
three points wind CCW, negative for CW, zero when colinear.

The error bound for f64 evaluation of this 2x2 determinant
scales with the magnitude of the products; we apply a relative
bound plus an absolute floor at `DEFAULT_TOL` for robustness
when both terms are tiny.

## Source
Lines 58–66 in `crates/oxide-sketch/src/geom/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-sketch/src/geom/predicates.md) |
