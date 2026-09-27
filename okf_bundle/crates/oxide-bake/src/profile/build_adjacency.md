---
okf_version: "0.2"
type: Function
title: build_adjacency
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/build_adjacency
language: rust
---

# build_adjacency

## Signature

```rust
fn build_adjacency(
    edges: &HashMap<SketchEntityId, &Entity>,
) -> HashMap<SketchEntityId, Vec<SketchEntityId>>
```

## Source
Lines 354–365 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| calls | [edge_endpoints](/crates/oxide-bake/src/profile/edge_endpoints.md) |
| called_by | [trace_closed_profile_entities](/crates/oxide-bake/src/profile/trace_closed_profile_entities.md) |
