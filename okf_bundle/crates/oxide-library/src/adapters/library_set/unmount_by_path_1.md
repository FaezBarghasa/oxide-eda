---
okf_version: "0.2"
type: Function
title: unmount_by_path
description: "Unmount the file-backed adapter at `path`, returning it."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/unmount_by_path_1
language: rust
---

# unmount_by_path

Unmount the file-backed adapter at `path`, returning it.

## Signature

```rust
pub fn unmount_by_path(&mut self, path: &Path) -> Option<Box<dyn LibraryAdapter>>
```

## Visibility

- `pub`

## Docstring

Unmount the file-backed adapter at `path`, returning it.

## Source
Lines 134–136 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
