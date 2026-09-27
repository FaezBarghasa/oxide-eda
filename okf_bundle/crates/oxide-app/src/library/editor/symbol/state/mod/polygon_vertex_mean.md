---
okf_version: "0.2"
type: Function
title: polygon_vertex_mean
description: "Plain vertex mean — [`polygon_centroid`]'s degenerate-ring fallback."
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/polygon_vertex_mean
language: rust
---

# polygon_vertex_mean

Plain vertex mean — [`polygon_centroid`]'s degenerate-ring fallback.

## Signature

```rust
fn polygon_vertex_mean(vertices: &[[f64; 2]]) -> [f64; 2]
```

## Docstring

Plain vertex mean — [`polygon_centroid`]'s degenerate-ring fallback.

## Source
Lines 395–401 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [polygon_centroid](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid.md) |
