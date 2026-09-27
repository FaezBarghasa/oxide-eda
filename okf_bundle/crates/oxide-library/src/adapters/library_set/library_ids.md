---
okf_version: "0.2"
type: Function
title: library_ids
description: "Iterate over `library_id`s of mounted libraries. Duplicates may"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/library_ids
language: rust
---

# library_ids

Iterate over `library_id`s of mounted libraries. Duplicates may

## Signature

```rust
impl LibrarySet { pub fn library_ids(&self) -> impl Iterator<Item = Uuid> + '_ }
```

## Visibility

- `pub`

## Docstring

Iterate over `library_id`s of mounted libraries. Duplicates may
appear when two file-backed adapters share an id.

## Source
Lines 173–175 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
