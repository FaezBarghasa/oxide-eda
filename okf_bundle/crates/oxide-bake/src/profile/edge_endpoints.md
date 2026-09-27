---
okf_version: "0.2"
type: Function
title: edge_endpoints
description: "Endpoint Points of an edge — Line `(start, end)` or Arc"
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/edge_endpoints
language: rust
---

# edge_endpoints

Endpoint Points of an edge — Line `(start, end)` or Arc

## Signature

```rust
fn edge_endpoints(entity: &Entity) -> Option<(SketchEntityId, SketchEntityId)>
```

## Docstring

Endpoint Points of an edge — Line `(start, end)` or Arc
`(start, end)` (the Arc's `center` Point is NOT a topology vertex).

## Source
Lines 369–375 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| called_by | [build_adjacency](/crates/oxide-bake/src/profile/build_adjacency.md) |
| called_by | [trace_closed_profile_entities](/crates/oxide-bake/src/profile/trace_closed_profile_entities.md) |
