---
okf_version: "0.2"
type: Function
title: commit
description: Flush pending writes; required before queries see new docs.
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/commit
language: rust
---

# commit

Flush pending writes; required before queries see new docs.

## Signature

```rust
impl TantivySearchIndex { pub fn commit(&self) -> Result<(), TantivyIndexError> }
```

## Visibility

- `pub`

## Docstring

Flush pending writes; required before queries see new docs.

## Source
Lines 366–372 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
