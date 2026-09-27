---
okf_version: "0.2"
type: Function
title: meshes_to_gltf
description: Build a minimal glTF 2.0 JSON and binary buffer from a flat list of meshes.
resource: crates/oxide-3d-model-importer/src/normalize/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-3d-model-importer/src/normalize/mod/meshes_to_gltf
language: rust
---

# meshes_to_gltf

Build a minimal glTF 2.0 JSON and binary buffer from a flat list of meshes.

## Signature

```rust
pub fn meshes_to_gltf(
    meshes: &[VrmlMesh],
    source_format: &str,
    source_path: &str,
    converter_version: &str,
) -> (Vec<u8>, Vec<u8>)
```

## Visibility

- `pub`

## Docstring

Build a minimal glTF 2.0 JSON and binary buffer from a flat list of meshes.

Output:
- `json_bytes`: UTF-8 encoded glTF JSON
- `bin_bytes`:  interleaved float32 position data

## Source
Lines 8–162 in `crates/oxide-3d-model-importer/src/normalize/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [normalize](/crates/oxide-3d-model-importer/src/normalize/mod.md) |
| calls | [compute_min_max](/crates/oxide-3d-model-importer/src/normalize/mod/compute_min_max.md) |
| called_by | [import_model](/crates/oxide-3d-model-importer/src/lib/import_model.md) |
| called_by | [empty_mesh_list_produces_valid_json](/crates/oxide-3d-model-importer/src/normalize/mod/empty_mesh_list_produces_valid_json.md) |
| called_by | [single_triangle_produces_one_mesh](/crates/oxide-3d-model-importer/src/normalize/mod/single_triangle_produces_one_mesh.md) |
