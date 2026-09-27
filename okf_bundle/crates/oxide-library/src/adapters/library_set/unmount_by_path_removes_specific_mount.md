---
okf_version: "0.2"
type: Function
title: unmount_by_path_removes_specific_mount
description: "`unmount_by_path` removes a specific file-backed mount when"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/unmount_by_path_removes_specific_mount
language: rust
---

# unmount_by_path_removes_specific_mount

`unmount_by_path` removes a specific file-backed mount when

## Signature

```rust
fn unmount_by_path_removes_specific_mount()
```

## Decorators

- `test`

## Docstring

`unmount_by_path` removes a specific file-backed mount when
callers can't disambiguate by `library_id` alone.
[test]

## Source
Lines 669–687 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
