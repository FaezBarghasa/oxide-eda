---
okf_version: "0.2"
type: Function
title: is_collinear
description: "`true` when every vertex in `ring` lies within `eps` mm of the"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/is_collinear
language: rust
---

# is_collinear

`true` when every vertex in `ring` lies within `eps` mm of the

## Signature

```rust
fn is_collinear(ring: &[[f64; 2]], eps: f64) -> bool
```

## Docstring

`true` when every vertex in `ring` lies within `eps` mm of the
infinite line through the first two DISTINCT vertices — a
genuinely degenerate (zero-width) ring, the case this gate is
documented to catch.

Deliberately NOT a zero-net-shoelace-area test (what this replaced):
a self-intersecting ring whose crossed lobes cancel to ~zero NET
area — a bowtie, e.g. `(0,0), (1.27,1.27), (1.27,0), (0,1.27)` —
has real 2D extent and renders even-odd; the user drew it on
purpose and it must commit, not be silently discarded as if it
were a straight line. Requires `ring.len() >= 2` (guaranteed by the
`< 3` length gate that runs immediately before this).

## Source
Lines 444–459 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [dist_sq](/crates/oxide-library/src/primitive/symbol/chain/dist_sq.md) |
| called_by | [finalize_ring](/crates/oxide-library/src/primitive/symbol/chain/finalize_ring.md) |
