---
okf_version: "0.2"
type: Function
title: library_paths
description: Iterate over the file paths of file-backed mounts. Path-less
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/library_paths_1
language: rust
---

# library_paths

Iterate over the file paths of file-backed mounts. Path-less

## Signature

```rust
pub fn library_paths(&self) -> impl Iterator<Item = &Path> + '_
```

## Visibility

- `pub`

## Docstring

Iterate over the file paths of file-backed mounts. Path-less
adapters are skipped.

## Source
Lines 179–184 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
