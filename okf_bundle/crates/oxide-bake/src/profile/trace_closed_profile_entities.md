---
okf_version: "0.2"
type: Function
title: trace_closed_profile_entities
description: "Trace a closed boundary starting at `start`, returning its"
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/trace_closed_profile_entities
language: rust
---

# trace_closed_profile_entities

Trace a closed boundary starting at `start`, returning its

## Signature

```rust
pub fn trace_closed_profile_entities(
    sketch: &SketchData,
    start: SketchEntityId,
) -> Result<ProfileEntities, TraceError>
```

## Visibility

- `pub`

## Docstring

Trace a closed boundary starting at `start`, returning its
entity topology.

Algorithm:
1. Build endpoint → list-of-edge adjacency over non-construction
Lines (Arcs / Circles are rejected with the matching error).
2. Pick an arbitrary endpoint of `start` as the loop anchor.
3. Walk: from the current endpoint, find the unique non-visited
incident edge (excluding the entity we just came from). If
there's exactly one, advance; otherwise return Branching /
OpenChain.
4. Loop closes when the next endpoint equals the anchor.

This is the single loop-walk implementation;
[`trace_closed_profile`] is a position mapping over its result.

## Source
Lines 113–188 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| calls | [collect_edges](/crates/oxide-bake/src/profile/collect_edges.md) |
| calls | [build_adjacency](/crates/oxide-bake/src/profile/build_adjacency.md) |
| calls | [edge_endpoints](/crates/oxide-bake/src/profile/edge_endpoints.md) |
| calls | [profile_points](/crates/oxide-bake/src/profile/profile_points.md) |
| called_by | [translate_profile_with_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/translate_profile_with_pad.md) |
| called_by | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
