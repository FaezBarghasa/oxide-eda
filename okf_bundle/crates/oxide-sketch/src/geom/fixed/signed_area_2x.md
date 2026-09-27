---
okf_version: "0.2"
type: Function
title: signed_area_2x
description: "Fixed-point variant of `signed_area`. Returns `2 * signed_area`"
resource: crates/oxide-sketch/src/geom/fixed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/fixed/signed_area_2x
language: rust
---

# signed_area_2x

Fixed-point variant of `signed_area`. Returns `2 * signed_area`

## Signature

```rust
pub fn signed_area_2x(points: &[FixPoint2]) -> i128
```

## Visibility

- `pub`

## Docstring

Fixed-point variant of `signed_area`. Returns `2 * signed_area`
in fixed-point units (the doubled form keeps it integer).

## Source
Lines 55–66 in `crates/oxide-sketch/src/geom/fixed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fixed](/crates/oxide-sketch/src/geom/fixed.md) |
| called_by | [signed_area_unit_square_is_2_times_one](/crates/oxide-sketch/src/geom/fixed/signed_area_unit_square_is_2_times_one.md) |
