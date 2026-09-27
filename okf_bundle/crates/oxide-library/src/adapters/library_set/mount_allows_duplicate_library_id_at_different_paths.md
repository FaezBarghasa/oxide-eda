---
okf_version: "0.2"
type: Function
title: mount_allows_duplicate_library_id_at_different_paths
description: Two file-backed adapters sharing an id but at different paths
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/mount_allows_duplicate_library_id_at_different_paths
language: rust
---

# mount_allows_duplicate_library_id_at_different_paths

Two file-backed adapters sharing an id but at different paths

## Signature

```rust
fn mount_allows_duplicate_library_id_at_different_paths()
```

## Decorators

- `test`

## Docstring

Two file-backed adapters sharing an id but at different paths
are allowed under the new model — the Components Panel dedups
at display time.
[test]

## Source
Lines 636–648 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
