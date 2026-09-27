---
okf_version: "0.2"
type: Function
title: build_mesh
description: Convert an IndexedFaceSet (positions + face-separated indices) to a triangle mesh.
resource: crates/oxide-3d-model-importer/src/vrml/parser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/mod/build_mesh
language: rust
---

# build_mesh

Convert an IndexedFaceSet (positions + face-separated indices) to a triangle mesh.

## Signature

```rust
fn build_mesh(
    positions: &[[f32; 3]],
    face_indices: &[i32],
    color: [f32; 4],
    xf: &Transform,
) -> VrmlMesh
```

## Docstring

Convert an IndexedFaceSet (positions + face-separated indices) to a triangle mesh.

## Source
Lines 150–202 in `crates/oxide-3d-model-importer/src/vrml/parser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-3d-model-importer/src/vrml/parser/mod.md) |
| called_by | [collect_meshes](/crates/oxide-3d-model-importer/src/vrml/parser/mod/collect_meshes.md) |
