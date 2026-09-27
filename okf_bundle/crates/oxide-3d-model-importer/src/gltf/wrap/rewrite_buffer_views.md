---
okf_version: "0.2"
type: Function
title: rewrite_buffer_views
resource: crates/oxide-3d-model-importer/src/gltf/wrap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/wrap/rewrite_buffer_views
language: rust
---

# rewrite_buffer_views

## Signature

```rust
fn rewrite_buffer_views(
    root: &mut Value,
    buffer_offsets: &[usize],
    source_path: &Path,
) -> Result<(), ModelImportError>
```

## Source
Lines 153–190 in `crates/oxide-3d-model-importer/src/gltf/wrap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wrap](/crates/oxide-3d-model-importer/src/gltf/wrap.md) |
| called_by | [wrap_gltf](/crates/oxide-3d-model-importer/src/gltf/wrap/wrap_gltf.md) |
