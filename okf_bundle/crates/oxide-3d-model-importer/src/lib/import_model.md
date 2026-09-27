---
okf_version: "0.2"
type: Function
title: import_model
description: "Synchronous entry point: convert `request.source_path` → cached GLB."
resource: crates/oxide-3d-model-importer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/lib/import_model
language: rust
---

# import_model

Synchronous entry point: convert `request.source_path` → cached GLB.

## Signature

```rust
pub fn import_model(request: ModelImportRequest) -> Result<ModelImportResult, ModelImportError>
```

## Visibility

- `pub`

## Docstring

Synchronous entry point: convert `request.source_path` → cached GLB.

On cache hit the existing GLB is returned without re-converting.

## Source
Lines 61–182 in `crates/oxide-3d-model-importer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-3d-model-importer/src/lib.md) |
| calls | [detect_format](/crates/oxide-3d-model-importer/src/lib/detect_format.md) |
| calls | [metadata](/crates/oxide-output/src/substitution/metadata.md) |
| calls | [cache_path](/crates/oxide-3d-model-importer/src/cache/cache_path.md) |
| calls | [is_cache_valid](/crates/oxide-3d-model-importer/src/cache/is_cache_valid.md) |
| calls | [meshes_to_gltf](/crates/oxide-3d-model-importer/src/normalize/mod/meshes_to_gltf.md) |
| calls | [write_glb](/crates/oxide-3d-model-importer/src/glb/writer/write_glb.md) |
| called_by | [import_cache_hit_on_second_call](/crates/oxide-3d-model-importer/tests/import_integration/import_cache_hit_on_second_call.md) |
| called_by | [import_cache_miss_on_version_bump](/crates/oxide-3d-model-importer/tests/import_integration/import_cache_miss_on_version_bump.md) |
| called_by | [import_missing_source_returns_error](/crates/oxide-3d-model-importer/tests/import_integration/import_missing_source_returns_error.md) |
| called_by | [import_tier0_glb_has_valid_magic](/crates/oxide-3d-model-importer/tests/import_integration/import_tier0_glb_has_valid_magic.md) |
| called_by | [import_tier0_gltf_produces_glb](/crates/oxide-3d-model-importer/tests/import_integration/import_tier0_gltf_produces_glb.md) |
| called_by | [import_tier0_step_produces_glb](/crates/oxide-3d-model-importer/tests/import_integration/import_tier0_step_produces_glb.md) |
| called_by | [import_tier0_vrml_produces_glb](/crates/oxide-3d-model-importer/tests/import_integration/import_tier0_vrml_produces_glb.md) |
| called_by | [import_tier1_two_body_produces_two_meshes](/crates/oxide-3d-model-importer/tests/import_integration/import_tier1_two_body_produces_two_meshes.md) |
| called_by | [import_unsupported_format_returns_error](/crates/oxide-3d-model-importer/tests/import_integration/import_unsupported_format_returns_error.md) |
