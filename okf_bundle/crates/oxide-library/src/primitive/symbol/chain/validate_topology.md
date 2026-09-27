---
okf_version: "0.2"
type: Function
title: validate_topology
description: "Confirm `clusters` describes exactly one simple cycle spanning all"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/validate_topology
language: rust
---

# validate_topology

Confirm `clusters` describes exactly one simple cycle spanning all

## Signature

```rust
fn validate_topology(n: usize, clusters: &EndpointClusters) -> Result<(), ChainError>
```

## Docstring

Confirm `clusters` describes exactly one simple cycle spanning all
`n` segments: no node touched by more than two segment-ends
([`ChainError::Branching`]), exactly one connected component
([`ChainError::Disjoint`] otherwise), and no loose ends
([`ChainError::OpenChain`] otherwise). `Ok(())` means
[`walk_cycle`] can run.

## Source
Lines 315–366 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [average_point](/crates/oxide-library/src/primitive/symbol/chain/average_point.md) |
| calls | [uf_union](/crates/oxide-library/src/primitive/symbol/chain/uf_union.md) |
| calls | [uf_find](/crates/oxide-library/src/primitive/symbol/chain/uf_find.md) |
| calls | [dist_sq](/crates/oxide-library/src/primitive/symbol/chain/dist_sq.md) |
| called_by | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
