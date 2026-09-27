---
okf_version: "0.2"
type: Function
title: embed_image_uris
resource: crates/oxide-3d-model-importer/src/gltf/wrap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/wrap/embed_image_uris
language: rust
---

# embed_image_uris

## Signature

```rust
fn embed_image_uris(root: &mut Value, base_dir: &Path, warnings: &mut Vec<ImportWarning>)
```

## Source
Lines 192–220 in `crates/oxide-3d-model-importer/src/gltf/wrap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wrap](/crates/oxide-3d-model-importer/src/gltf/wrap.md) |
| calls | [guess_mime_type](/crates/oxide-3d-model-importer/src/gltf/wrap/guess_mime_type.md) |
| called_by | [wrap_gltf](/crates/oxide-3d-model-importer/src/gltf/wrap/wrap_gltf.md) |
