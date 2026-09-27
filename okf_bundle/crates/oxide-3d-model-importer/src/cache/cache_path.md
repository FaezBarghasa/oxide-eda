---
okf_version: "0.2"
type: Function
title: cache_path
description: Compute the SHA-256-based cache path for a source file.
resource: crates/oxide-3d-model-importer/src/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:25:27Z"
concept_id: crates/oxide-3d-model-importer/src/cache/cache_path
language: rust
---

# cache_path

Compute the SHA-256-based cache path for a source file.

## Signature

```rust
pub fn cache_path(
    cache_dir: &Path,
    source_path: &Path,
    source_mtime: SystemTime,
    converter_version: &str,
) -> Result<PathBuf, ModelImportError>
```

## Visibility

- `pub`

## Docstring

Compute the SHA-256-based cache path for a source file.

Cache key = sha256(absolute_path_str + "|" + mtime_unix_sec_str + "|" + converter_version)

## Source
Lines 11–37 in `crates/oxide-3d-model-importer/src/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-3d-model-importer/src/cache.md) |
| called_by | [cache_path_differs_on_mtime_change](/crates/oxide-3d-model-importer/src/cache/cache_path_differs_on_mtime_change.md) |
| called_by | [cache_path_differs_on_version_change](/crates/oxide-3d-model-importer/src/cache/cache_path_differs_on_version_change.md) |
| called_by | [cache_path_has_glb_extension](/crates/oxide-3d-model-importer/src/cache/cache_path_has_glb_extension.md) |
| called_by | [cache_path_is_deterministic](/crates/oxide-3d-model-importer/src/cache/cache_path_is_deterministic.md) |
| called_by | [import_model](/crates/oxide-3d-model-importer/src/lib/import_model.md) |
