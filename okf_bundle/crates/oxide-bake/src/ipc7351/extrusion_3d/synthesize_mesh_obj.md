---
okf_version: "0.2"
type: Function
title: synthesize_mesh_obj
description: Procedurally extrudes a 3D boundary-representation mesh (Wavefront OBJ bytes).
resource: crates/oxide-bake/src/ipc7351/extrusion_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:05:10Z"
concept_id: crates/oxide-bake/src/ipc7351/extrusion_3d/synthesize_mesh_obj
language: rust
---

# synthesize_mesh_obj

Procedurally extrudes a 3D boundary-representation mesh (Wavefront OBJ bytes).

## Signature

```rust
impl Package3DExtruder { pub fn synthesize_mesh_obj(dimensions: &PackageDimensions) -> Vec<u8> }
```

## Visibility

- `pub`

## Docstring

Procedurally extrudes a 3D boundary-representation mesh (Wavefront OBJ bytes).
Centroid is strictly anchored to (0, 0, 0) matching footprint origin.

## Source
Lines 39–87 in `crates/oxide-bake/src/ipc7351/extrusion_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [extrusion_3d](/crates/oxide-bake/src/ipc7351/extrusion_3d.md) |
