---
okf_version: "0.2"
type: Function
title: decode_data_uri
resource: crates/oxide-3d-model-importer/src/gltf/wrap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/gltf/wrap/decode_data_uri
language: rust
---

# decode_data_uri

## Signature

```rust
fn decode_data_uri(uri: &str) -> Option<Vec<u8>>
```

## Source
Lines 246–255 in `crates/oxide-3d-model-importer/src/gltf/wrap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wrap](/crates/oxide-3d-model-importer/src/gltf/wrap.md) |
| called_by | [decode_data_uri_base64_ok](/crates/oxide-3d-model-importer/src/gltf/wrap/decode_data_uri_base64_ok.md) |
| called_by | [read_buffer_payload](/crates/oxide-3d-model-importer/src/gltf/wrap/read_buffer_payload.md) |
