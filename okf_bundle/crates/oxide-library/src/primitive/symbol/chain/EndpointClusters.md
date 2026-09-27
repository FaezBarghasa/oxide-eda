---
okf_version: "0.2"
type: Class
title: EndpointClusters
description: "Endpoint-adjacency result of clustering every segment's start/end"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/EndpointClusters
language: rust
---

# EndpointClusters

Endpoint-adjacency result of clustering every segment's start/end

## Signature

```rust
struct EndpointClusters
```

## Docstring

Endpoint-adjacency result of clustering every segment's start/end
point. `node_entries`/`node_points` are keyed by cluster id (an
arbitrary but stable `usize`); `endpoint_node` maps a segment's own
`(segment_index, is_start)` endpoint directly to its cluster id.

## Methods

- `node_entries`
- `node_points`
- `endpoint_node`

## Source
Lines 254–258 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
