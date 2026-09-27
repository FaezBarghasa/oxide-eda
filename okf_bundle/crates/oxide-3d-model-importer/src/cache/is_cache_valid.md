---
okf_version: "0.2"
type: Function
title: is_cache_valid
description: "Returns `true` if a valid cached GLB already exists for this source file."
resource: crates/oxide-3d-model-importer/src/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:25:27Z"
concept_id: crates/oxide-3d-model-importer/src/cache/is_cache_valid
language: rust
---

# is_cache_valid

Returns `true` if a valid cached GLB already exists for this source file.

## Signature

```rust
pub fn is_cache_valid(glb_path: &Path) -> bool
```

## Visibility

- `pub`

## Docstring

Returns `true` if a valid cached GLB already exists for this source file.

## Source
Lines 40–42 in `crates/oxide-3d-model-importer/src/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-3d-model-importer/src/cache.md) |
| calls | [metadata](/crates/oxide-output/src/substitution/metadata.md) |
| called_by | [import_model](/crates/oxide-3d-model-importer/src/lib/import_model.md) |
