---
okf_version: "0.2"
type: Function
title: write_tier0_wrl
resource: crates/oxide-3d-model-importer/tests/import_integration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-3d-model-importer/tests/import_integration/write_tier0_wrl
language: rust
---

# write_tier0_wrl

## Signature

```rust
fn write_tier0_wrl(path: &std::path::Path)
```

## Source
Lines 3–17 in `crates/oxide-3d-model-importer/tests/import_integration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [import_integration](/crates/oxide-3d-model-importer/tests/import_integration.md) |
| called_by | [import_cache_hit_on_second_call](/crates/oxide-3d-model-importer/tests/import_integration/import_cache_hit_on_second_call.md) |
| called_by | [import_cache_miss_on_version_bump](/crates/oxide-3d-model-importer/tests/import_integration/import_cache_miss_on_version_bump.md) |
| called_by | [import_tier0_glb_has_valid_magic](/crates/oxide-3d-model-importer/tests/import_integration/import_tier0_glb_has_valid_magic.md) |
| called_by | [import_tier0_vrml_produces_glb](/crates/oxide-3d-model-importer/tests/import_integration/import_tier0_vrml_produces_glb.md) |
