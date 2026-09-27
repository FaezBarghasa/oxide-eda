---
okf_version: "0.2"
type: Function
title: wrap_gltf
description: "Wrap a `.gltf` JSON payload into GLB chunks (JSON + BIN)."
resource: crates/oxide-3d-model-importer/src/gltf/wrap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/wrap/wrap_gltf
language: rust
---

# wrap_gltf

Wrap a `.gltf` JSON payload into GLB chunks (JSON + BIN).

## Signature

```rust
pub fn wrap_gltf(
    source: &str,
    source_path: &Path,
    converter_version: &str,
) -> Result<GltfWrapResult, ModelImportError>
```

## Visibility

- `pub`

## Docstring

Wrap a `.gltf` JSON payload into GLB chunks (JSON + BIN).

This function keeps geometry indices/accessors intact while collapsing
multiple external buffers into a single BIN chunk and rewriting
`bufferViews[*].buffer`/`byteOffset` accordingly.

## Source
Lines 21–99 in `crates/oxide-3d-model-importer/src/gltf/wrap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wrap](/crates/oxide-3d-model-importer/src/gltf/wrap.md) |
| calls | [validate_asset_version](/crates/oxide-3d-model-importer/src/gltf/wrap/validate_asset_version.md) |
| calls | [align4](/crates/oxide-3d-model-importer/src/gltf/wrap/align4.md) |
| calls | [read_buffer_payload](/crates/oxide-3d-model-importer/src/gltf/wrap/read_buffer_payload.md) |
| calls | [rewrite_buffer_views](/crates/oxide-3d-model-importer/src/gltf/wrap/rewrite_buffer_views.md) |
| calls | [embed_image_uris](/crates/oxide-3d-model-importer/src/gltf/wrap/embed_image_uris.md) |
| calls | [rewrite_asset_extras](/crates/oxide-3d-model-importer/src/gltf/wrap/rewrite_asset_extras.md) |
| called_by | [load](/crates/oxide-3d-model-importer/src/gltf/mod/load.md) |
