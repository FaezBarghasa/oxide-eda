---
okf_version: "0.2"
type: Function
title: solves_near_collinear_triangle_in_the_drifted_epsilon_band
description: "Regression for the drifted-epsilon bug: a near-collinear triangle"
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/solves_near_collinear_triangle_in_the_drifted_epsilon_band
language: rust
---

# solves_near_collinear_triangle_in_the_drifted_epsilon_band

Regression for the drifted-epsilon bug: a near-collinear triangle

## Signature

```rust
fn solves_near_collinear_triangle_in_the_drifted_epsilon_band()
```

## Decorators

- `test`

## Docstring

Regression for the drifted-epsilon bug: a near-collinear triangle
whose determinant falls in (1e-12, 1e-9) is a solvable arc under
the canonical `1e-12` threshold, but was silently treated as
degenerate by the three call sites that used `1e-9`.
[test]

## Source
Lines 194–221 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
