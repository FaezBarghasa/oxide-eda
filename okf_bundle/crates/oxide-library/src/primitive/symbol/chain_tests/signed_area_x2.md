---
okf_version: "0.2"
type: Function
title: signed_area_x2
description: "Twice the signed polygon area (shoelace, standard math orientation —"
resource: crates/oxide-library/src/primitive/symbol/chain_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain_tests/signed_area_x2
language: rust
---

# signed_area_x2

Twice the signed polygon area (shoelace, standard math orientation —

## Signature

```rust
fn signed_area_x2(ring: &[[f64; 2]]) -> f64
```

## Docstring

Twice the signed polygon area (shoelace, standard math orientation —
positive = counter-clockwise).

## Source
Lines 31–40 in `crates/oxide-library/src/primitive/symbol/chain_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain_tests](/crates/oxide-library/src/primitive/symbol/chain_tests.md) |
| called_by | [cw_input_square_is_renormalised_to_ccw](/crates/oxide-library/src/primitive/symbol/chain_tests/cw_input_square_is_renormalised_to_ccw.md) |
| called_by | [self_intersecting_bowtie_with_net_zero_area_commits](/crates/oxide-library/src/primitive/symbol/chain_tests/self_intersecting_bowtie_with_net_zero_area_commits.md) |
