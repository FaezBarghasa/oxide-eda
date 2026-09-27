---
okf_version: "0.2"
type: Function
title: resolve_edge_loop_points
resource: crates/oxide-3d-model-importer/src/step/p21.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/p21/resolve_edge_loop_points
language: rust
---

# resolve_edge_loop_points

## Signature

```rust
fn resolve_edge_loop_points(
    oriented_edge_refs: &[u32],
    oriented_edges: &HashMap<u32, u32>,
    edge_curves: &HashMap<u32, (u32, u32)>,
    vertex_to_point: &HashMap<u32, u32>,
    points: &HashMap<u32, [f32; 3]>,
) -> Vec<[f32; 3]>
```

## Source
Lines 302–348 in `crates/oxide-3d-model-importer/src/step/p21.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [p21](/crates/oxide-3d-model-importer/src/step/p21.md) |
| called_by | [parse_to_meshes](/crates/oxide-3d-model-importer/src/step/p21/parse_to_meshes.md) |
