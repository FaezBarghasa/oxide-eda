---
okf_version: "0.2"
type: Function
title: load
description: "Load a `.gltf` JSON source and wrap it into a GLB payload."
resource: crates/oxide-3d-model-importer/src/gltf/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/mod/load
language: rust
---

# load

Load a `.gltf` JSON source and wrap it into a GLB payload.

## Signature

```rust
pub fn load(path: &PathBuf, converter_version: &str) -> Result<GltfWrapResult, ModelImportError>
```

## Visibility

- `pub`

## Docstring

Load a `.gltf` JSON source and wrap it into a GLB payload.

## Source
Lines 10–17 in `crates/oxide-3d-model-importer/src/gltf/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gltf](/crates/oxide-3d-model-importer/src/gltf/mod.md) |
| calls | [wrap_gltf](/crates/oxide-3d-model-importer/src/gltf/wrap/wrap_gltf.md) |
