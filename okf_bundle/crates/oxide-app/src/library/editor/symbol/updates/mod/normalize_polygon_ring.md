---
okf_version: "0.2"
type: Function
title: normalize_polygon_ring
description: "Normalise a click-collected vertex ring before committing it:"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_polygon_ring
language: rust
---

# normalize_polygon_ring

Normalise a click-collected vertex ring before committing it:

## Signature

```rust
fn normalize_polygon_ring(vertices: Vec<(f64, f64)>) -> Vec<[f64; 2]>
```

## Docstring

Normalise a click-collected vertex ring before committing it:

- Collapse consecutive epsilon-duplicate vertices (e.g. two slow
clicks landing on the same snapped point mid-sequence — `[P, P,
Q, R]` -> `[P, Q, R]`), including the wrap-around last-to-first
pair (a closing click landing back on vertex 0's snapped grid
position, which would otherwise double the closing edge at
render time). Mirrors `oxide_library`'s chain `finalize_ring`
dedup pass exactly, using the same `CHAIN_ENDPOINT_EPSILON_MM`
constant, so this click-collect commit path and the
Join-into-Polygon chain path agree on what counts as "the same
point."
- Reject a degenerate ring — every vertex collinear — by returning
an empty Vec, which the caller's `>= 3` check then discards. A
`<3`-vertex input (before or after dedup) returns empty
unconditionally. Deliberately NOT a zero-net-shoelace-area test:
see [`polygon_is_collinear`]'s doc comment for why a
self-intersecting-but-non-collinear ring (a bowtie) must still
commit.

## Source
Lines 186–196 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [collapse_consecutive_duplicate_vertices](/crates/oxide-app/src/library/editor/symbol/updates/mod/collapse_consecutive_duplicate_vertices.md) |
| calls | [polygon_is_collinear](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_is_collinear.md) |
| called_by | [commit_or_discard_polygon](/crates/oxide-app/src/library/editor/symbol/updates/mod/commit_or_discard_polygon.md) |
