---
okf_version: "0.2"
type: Function
title: polyline_length
description: Total length of a tessellated polyline — the sum of consecutive
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/polyline_length
language: rust
---

# polyline_length

Total length of a tessellated polyline — the sum of consecutive

## Signature

```rust
fn polyline_length(poly: &[[f64; 2]]) -> f64
```

## Docstring

Total length of a tessellated polyline — the sum of consecutive
point-to-point distances. For a `Line` (a 2-point polyline) this is
exactly its chord length; for a tessellated `Arc` it's the polygonal
approximation of the true arc length. See
[`reject_sub_epsilon_segments`] for why this, not the chord between
the segment's own two endpoints, is the right degeneracy gate.

## Source
Lines 246–248 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [dist_sq](/crates/oxide-library/src/primitive/symbol/chain/dist_sq.md) |
| called_by | [reject_sub_epsilon_segments](/crates/oxide-library/src/primitive/symbol/chain/reject_sub_epsilon_segments.md) |
