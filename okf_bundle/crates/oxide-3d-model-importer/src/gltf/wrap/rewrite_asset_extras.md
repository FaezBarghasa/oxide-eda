---
okf_version: "0.2"
type: Function
title: rewrite_asset_extras
resource: crates/oxide-3d-model-importer/src/gltf/wrap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/wrap/rewrite_asset_extras
language: rust
---

# rewrite_asset_extras

## Signature

```rust
fn rewrite_asset_extras(root: &mut Value, source_path: &Path, converter_version: &str)
```

## Source
Lines 222–244 in `crates/oxide-3d-model-importer/src/gltf/wrap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wrap](/crates/oxide-3d-model-importer/src/gltf/wrap.md) |
| called_by | [wrap_gltf](/crates/oxide-3d-model-importer/src/gltf/wrap/wrap_gltf.md) |
