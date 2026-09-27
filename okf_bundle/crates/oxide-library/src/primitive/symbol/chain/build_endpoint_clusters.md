---
okf_version: "0.2"
type: Function
title: build_endpoint_clusters
description: "Cluster the `2n` segment endpoints (start + end of every segment)"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/build_endpoint_clusters
language: rust
---

# build_endpoint_clusters

Cluster the `2n` segment endpoints (start + end of every segment)

## Signature

```rust
fn build_endpoint_clusters(polylines: &[Vec<[f64; 2]>]) -> EndpointClusters
```

## Docstring

Cluster the `2n` segment endpoints (start + end of every segment)
into nodes — points within [`CHAIN_ENDPOINT_EPSILON_MM`] of each
other, transitively (see the [`chain_into_closed_contour`] doc
comment) — via a small union-find.

## Source
Lines 264–307 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [dist_sq](/crates/oxide-library/src/primitive/symbol/chain/dist_sq.md) |
| calls | [uf_union](/crates/oxide-library/src/primitive/symbol/chain/uf_union.md) |
| calls | [ref_index](/crates/oxide-library/src/primitive/symbol/chain/ref_index.md) |
| calls | [uf_find](/crates/oxide-library/src/primitive/symbol/chain/uf_find.md) |
| called_by | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
