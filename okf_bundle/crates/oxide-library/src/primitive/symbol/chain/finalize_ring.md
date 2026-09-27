---
okf_version: "0.2"
type: Function
title: finalize_ring
description: "Collapse consecutive (and wrap-around) duplicate points, reject"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/finalize_ring
language: rust
---

# finalize_ring

Collapse consecutive (and wrap-around) duplicate points, reject

## Signature

```rust
fn finalize_ring(raw: Vec<[f64; 2]>) -> Result<Vec<[f64; 2]>, ChainError>
```

## Docstring

Collapse consecutive (and wrap-around) duplicate points, reject
degenerate results, and normalise winding to CCW.

## Source
Lines 405–430 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [dist_sq](/crates/oxide-library/src/primitive/symbol/chain/dist_sq.md) |
| calls | [is_collinear](/crates/oxide-library/src/primitive/symbol/chain/is_collinear.md) |
| calls | [signed_area_x2](/crates/oxide-library/src/primitive/symbol/chain/signed_area_x2.md) |
| called_by | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
