---
okf_version: "0.2"
type: Function
title: compute_min_max
resource: crates/oxide-3d-model-importer/src/normalize/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-3d-model-importer/src/normalize/mod/compute_min_max
language: rust
---

# compute_min_max

## Signature

```rust
fn compute_min_max(positions: &[f32]) -> ([f32; 3], [f32; 3])
```

## Source
Lines 164–178 in `crates/oxide-3d-model-importer/src/normalize/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [normalize](/crates/oxide-3d-model-importer/src/normalize/mod.md) |
| called_by | [meshes_to_gltf](/crates/oxide-3d-model-importer/src/normalize/mod/meshes_to_gltf.md) |
