---
okf_version: "0.2"
type: Function
title: project_referencing
description: "A `ProjectData` whose `libraries` are absolute `Shared` entries, so"
resource: crates/oxide-app/tests/library_open_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:14Z"
concept_id: crates/oxide-app/tests/library_open_cache/project_referencing
language: rust
---

# project_referencing

A `ProjectData` whose `libraries` are absolute `Shared` entries, so

## Signature

```rust
fn project_referencing(dir: &Path, libs: &[PathBuf]) -> ProjectData
```

## Docstring

A `ProjectData` whose `libraries` are absolute `Shared` entries, so
`resolve_library_path` hands back exactly the paths given —
`open_library` matches its already-open check on `==`, so any rewriting
here would silently turn the warm path back into a cold one.

## Source
Lines 250–270 in `crates/oxide-app/tests/library_open_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_open_cache](/crates/oxide-app/tests/library_open_cache.md) |
| called_by | [auto_mount_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/auto_mount_refreshes_an_already_mounted_library.md) |
| called_by | [create_library_at_mounts_with_primed_empty_caches](/crates/oxide-app/tests/library_open_cache/create_library_at_mounts_with_primed_empty_caches.md) |
