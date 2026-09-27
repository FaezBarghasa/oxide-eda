---
okf_version: "0.2"
type: Function
title: build_query
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/build_query
language: rust
---

# build_query

## Signature

```rust
impl TantivySearchIndex { fn build_query(&self, q: &SearchQuery) -> Result<Box<dyn Query>, TantivyIndexError> }
```

## Source
Lines 374–451 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| calls | [literal_phrase_query](/crates/oxide-library/src/search_index/literal_phrase_query.md) |
