---
okf_version: "0.2"
type: Function
title: polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie
description: A degenerate ring (~zero signed area — a bowtie) falls back to the
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie
language: rust
---

# polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie

A degenerate ring (~zero signed area — a bowtie) falls back to the

## Signature

```rust
fn polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie()
```

## Decorators

- `test`

## Docstring

A degenerate ring (~zero signed area — a bowtie) falls back to the
plain vertex mean rather than dividing by ~zero.
[test]

## Source
Lines 615–625 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| calls | [polygon_centroid](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid.md) |
