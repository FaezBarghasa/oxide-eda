---
okf_version: "0.2"
type: Function
title: library_paths_skips_pathless_mounts
description: "`library_paths` lists file-backed mounts only; path-less"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/library_paths_skips_pathless_mounts
language: rust
---

# library_paths_skips_pathless_mounts

`library_paths` lists file-backed mounts only; path-less

## Signature

```rust
fn library_paths_skips_pathless_mounts()
```

## Decorators

- `test`

## Docstring

`library_paths` lists file-backed mounts only; path-less
adapters are filtered out.
[test]

## Source
Lines 692–703 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
