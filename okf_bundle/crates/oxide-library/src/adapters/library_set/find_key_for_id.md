---
okf_version: "0.2"
type: Function
title: find_key_for_id
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/find_key_for_id
language: rust
---

# find_key_for_id

## Signature

```rust
impl LibrarySet { fn find_key_for_id(&self, library_id: Uuid) -> Option<MountKey> }
```

## Source
Lines 246–251 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
