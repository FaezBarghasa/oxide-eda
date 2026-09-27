---
okf_version: "0.2"
type: Function
title: read_buffer_payload
resource: crates/oxide-3d-model-importer/src/gltf/wrap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/wrap/read_buffer_payload
language: rust
---

# read_buffer_payload

## Signature

```rust
fn read_buffer_payload(
    source_path: &Path,
    base_dir: &Path,
    index: usize,
    buffer: &Value,
) -> Result<Vec<u8>, ModelImportError>
```

## Source
Lines 121–151 in `crates/oxide-3d-model-importer/src/gltf/wrap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wrap](/crates/oxide-3d-model-importer/src/gltf/wrap.md) |
| calls | [decode_data_uri](/crates/oxide-3d-model-importer/src/gltf/wrap/decode_data_uri.md) |
| called_by | [wrap_gltf](/crates/oxide-3d-model-importer/src/gltf/wrap/wrap_gltf.md) |
