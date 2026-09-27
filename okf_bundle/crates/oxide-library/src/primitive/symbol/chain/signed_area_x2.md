---
okf_version: "0.2"
type: Function
title: signed_area_x2
description: "Twice the signed polygon area (shoelace, standard math orientation —"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/signed_area_x2
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
positive = counter-clockwise). Doubled to skip a division that every
call site only needs the sign/magnitude of.

## Source
Lines 523–532 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| called_by | [finalize_ring](/crates/oxide-library/src/primitive/symbol/chain/finalize_ring.md) |
