---
okf_version: "0.2"
type: Function
title: triangulate_polygon
resource: crates/oxide-3d-model-importer/src/step/p21.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/p21/triangulate_polygon
language: rust
---

# triangulate_polygon

## Signature

```rust
fn triangulate_polygon(points: &[[f32; 3]]) -> VrmlMesh
```

## Source
Lines 350–376 in `crates/oxide-3d-model-importer/src/step/p21.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [p21](/crates/oxide-3d-model-importer/src/step/p21.md) |
| called_by | [parse_to_meshes](/crates/oxide-3d-model-importer/src/step/p21/parse_to_meshes.md) |
