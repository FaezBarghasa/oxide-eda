---
okf_version: "0.2"
type: Function
title: writer
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/writer_1
language: rust
---

# writer

## Signature

```rust
fn writer(&self) -> Result<MutexGuard<'_, IndexWriter>, TantivyIndexError>
```

## Source
Lines 275–282 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
