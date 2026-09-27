---
okf_version: "0.2"
type: Function
title: collapse_consecutive_duplicate_vertices
description: "Collapse consecutive epsilon-duplicate points, including the"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/collapse_consecutive_duplicate_vertices
language: rust
---

# collapse_consecutive_duplicate_vertices

Collapse consecutive epsilon-duplicate points, including the

## Signature

```rust
fn collapse_consecutive_duplicate_vertices(raw: Vec<[f64; 2]>) -> Vec<[f64; 2]>
```

## Docstring

Collapse consecutive epsilon-duplicate points, including the
wrap-around last-to-first pair — mirrors `oxide_library`'s chain
`finalize_ring` dedup pass exactly.

## Source
Lines 201–215 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [dist_sq](/crates/oxide-app/src/library/editor/symbol/updates/mod/dist_sq.md) |
| called_by | [normalize_polygon_ring](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_polygon_ring.md) |
