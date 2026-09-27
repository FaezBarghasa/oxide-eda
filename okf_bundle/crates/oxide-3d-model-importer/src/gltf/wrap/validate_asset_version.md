---
okf_version: "0.2"
type: Function
title: validate_asset_version
resource: crates/oxide-3d-model-importer/src/gltf/wrap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/wrap/validate_asset_version
language: rust
---

# validate_asset_version

## Signature

```rust
fn validate_asset_version(root: &Value, source_path: &Path) -> Result<(), ModelImportError>
```

## Source
Lines 101–119 in `crates/oxide-3d-model-importer/src/gltf/wrap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wrap](/crates/oxide-3d-model-importer/src/gltf/wrap.md) |
| called_by | [wrap_gltf](/crates/oxide-3d-model-importer/src/gltf/wrap/wrap_gltf.md) |
