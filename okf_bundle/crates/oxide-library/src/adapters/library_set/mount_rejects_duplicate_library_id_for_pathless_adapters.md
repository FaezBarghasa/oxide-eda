---
okf_version: "0.2"
type: Function
title: mount_rejects_duplicate_library_id_for_pathless_adapters
description: Path-less adapters (DB/in-memory) keep the legacy duplicate-id
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/mount_rejects_duplicate_library_id_for_pathless_adapters
language: rust
---

# mount_rejects_duplicate_library_id_for_pathless_adapters

Path-less adapters (DB/in-memory) keep the legacy duplicate-id

## Signature

```rust
fn mount_rejects_duplicate_library_id_for_pathless_adapters()
```

## Decorators

- `test`

## Docstring

Path-less adapters (DB/in-memory) keep the legacy duplicate-id
safety check — two of those with the same id is a config bug.
[test]

## Source
Lines 623–630 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
