---
okf_version: "0.2"
type: Function
title: write_glb
description: Serializes a GLB 2.0 binary container from a JSON chunk and an optional
resource: crates/oxide-3d-model-importer/src/glb/writer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/glb/writer/write_glb
language: rust
---

# write_glb

Serializes a GLB 2.0 binary container from a JSON chunk and an optional

## Signature

```rust
pub fn write_glb(json_bytes: &[u8], bin_bytes: &[u8]) -> Result<Vec<u8>, ModelImportError>
```

## Visibility

- `pub`

## Docstring

Serializes a GLB 2.0 binary container from a JSON chunk and an optional
binary buffer chunk. Follows the glTF 2.0 specification (Khronos, Apache 2.0).

GLB layout:
12-byte header | JSON chunk (padded to 4 bytes) | BIN chunk (optional, padded)

## Source
Lines 8–45 in `crates/oxide-3d-model-importer/src/glb/writer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [writer](/crates/oxide-3d-model-importer/src/glb/writer.md) |
| calls | [pad4](/crates/oxide-3d-model-importer/src/glb/writer/pad4.md) |
| called_by | [glb_header_magic_and_version](/crates/oxide-3d-model-importer/src/glb/writer/glb_header_magic_and_version.md) |
| called_by | [glb_json_chunk_type_is_correct](/crates/oxide-3d-model-importer/src/glb/writer/glb_json_chunk_type_is_correct.md) |
| called_by | [glb_json_padded_to_4_byte_boundary](/crates/oxide-3d-model-importer/src/glb/writer/glb_json_padded_to_4_byte_boundary.md) |
| called_by | [glb_total_length_matches_byte_count](/crates/oxide-3d-model-importer/src/glb/writer/glb_total_length_matches_byte_count.md) |
| called_by | [glb_with_bin_chunk_has_correct_structure](/crates/oxide-3d-model-importer/src/glb/writer/glb_with_bin_chunk_has_correct_structure.md) |
| called_by | [import_model](/crates/oxide-3d-model-importer/src/lib/import_model.md) |
