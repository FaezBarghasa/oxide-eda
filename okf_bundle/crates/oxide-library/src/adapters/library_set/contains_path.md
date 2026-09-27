---
okf_version: "0.2"
type: Function
title: contains_path
description: "True if a file-backed adapter is mounted at `path`."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/contains_path
language: rust
---

# contains_path

True if a file-backed adapter is mounted at `path`.

## Signature

```rust
impl LibrarySet { pub fn contains_path(&self, path: &Path) -> bool }
```

## Visibility

- `pub`

## Docstring

True if a file-backed adapter is mounted at `path`.

## Source
Lines 154–156 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
