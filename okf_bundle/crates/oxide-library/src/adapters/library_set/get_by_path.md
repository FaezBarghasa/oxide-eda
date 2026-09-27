---
okf_version: "0.2"
type: Function
title: get_by_path
description: "Borrow the file-backed adapter mounted at `path`, if any."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/get_by_path
language: rust
---

# get_by_path

Borrow the file-backed adapter mounted at `path`, if any.

## Signature

```rust
impl LibrarySet { pub fn get_by_path(&self, path: &Path) -> Option<&dyn LibraryAdapter> }
```

## Visibility

- `pub`

## Docstring

Borrow the file-backed adapter mounted at `path`, if any.

## Source
Lines 165–169 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
