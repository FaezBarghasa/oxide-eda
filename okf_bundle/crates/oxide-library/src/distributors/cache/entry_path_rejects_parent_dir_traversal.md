---
okf_version: "0.2"
type: Function
title: entry_path_rejects_parent_dir_traversal
description: "M2: an MPN containing `..` must be rejected on every cache op so a"
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/entry_path_rejects_parent_dir_traversal
language: rust
---

# entry_path_rejects_parent_dir_traversal

M2: an MPN containing `..` must be rejected on every cache op so a

## Signature

```rust
fn entry_path_rejects_parent_dir_traversal()
```

## Decorators

- `test`

## Docstring

M2: an MPN containing `..` must be rejected on every cache op so a
vendor-supplied response can't escape the cache root and overwrite
arbitrary files.
[test]

## Source
Lines 201–216 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
| calls | [part](/crates/oxide-library/src/distributors/cache/part.md) |
