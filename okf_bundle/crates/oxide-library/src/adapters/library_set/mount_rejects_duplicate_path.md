---
okf_version: "0.2"
type: Function
title: mount_rejects_duplicate_path
description: "Two file-backed adapters at the *same* path collide — that's"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/mount_rejects_duplicate_path
language: rust
---

# mount_rejects_duplicate_path

Two file-backed adapters at the *same* path collide — that's

## Signature

```rust
fn mount_rejects_duplicate_path()
```

## Decorators

- `test`

## Docstring

Two file-backed adapters at the *same* path collide — that's
always a bug (mount the same .snxlib twice).
[test]

## Source
Lines 653–664 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
