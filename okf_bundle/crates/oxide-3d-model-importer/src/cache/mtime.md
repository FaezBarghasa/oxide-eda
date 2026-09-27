---
okf_version: "0.2"
type: Function
title: mtime
resource: crates/oxide-3d-model-importer/src/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:25:27Z"
concept_id: crates/oxide-3d-model-importer/src/cache/mtime
language: rust
---

# mtime

## Signature

```rust
fn mtime(secs: u64) -> SystemTime
```

## Source
Lines 49–51 in `crates/oxide-3d-model-importer/src/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-3d-model-importer/src/cache.md) |
| called_by | [cache_path_differs_on_mtime_change](/crates/oxide-3d-model-importer/src/cache/cache_path_differs_on_mtime_change.md) |
| called_by | [cache_path_differs_on_version_change](/crates/oxide-3d-model-importer/src/cache/cache_path_differs_on_version_change.md) |
| called_by | [cache_path_has_glb_extension](/crates/oxide-3d-model-importer/src/cache/cache_path_has_glb_extension.md) |
| called_by | [cache_path_is_deterministic](/crates/oxide-3d-model-importer/src/cache/cache_path_is_deterministic.md) |
