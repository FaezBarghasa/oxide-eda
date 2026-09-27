---
okf_version: "0.2"
type: Function
title: polygon_is_collinear
description: "`true` when every vertex in `points` lies within `eps` mm of the"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_is_collinear
language: rust
---

# polygon_is_collinear

`true` when every vertex in `points` lies within `eps` mm of the

## Signature

```rust
fn polygon_is_collinear(points: &[[f64; 2]], eps: f64) -> bool
```

## Docstring

`true` when every vertex in `points` lies within `eps` mm of the
infinite line through the first two DISTINCT vertices — mirrors
`oxide_library`'s chain `is_collinear` (same algorithm,
independently implemented: it's a private helper on the other side
of the crate boundary). A genuinely degenerate (zero-width) ring,
the case this gate is documented to catch.

Deliberately NOT a zero-net-shoelace-area test (what this
replaced): a self-intersecting ring whose crossed lobes cancel to
~zero NET area — a bowtie, e.g. `(0,0), (1.27,1.27), (1.27,0),
(0,1.27)` — has real 2D extent and renders even-odd; the user drew
it on purpose and it must commit, not be silently discarded as if
it were a straight line. Requires `points.len() >= 2` (guaranteed
by the `< 3` length gate that runs immediately before this).

## Source
Lines 237–256 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [dist_sq](/crates/oxide-app/src/library/editor/symbol/updates/mod/dist_sq.md) |
| called_by | [normalize_polygon_ring](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_polygon_ring.md) |
