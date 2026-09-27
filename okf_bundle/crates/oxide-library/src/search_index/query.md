---
okf_version: "0.2"
type: Function
title: query
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/query
language: rust
---

# query

## Signature

```rust
impl TantivySearchIndex { fn query(&self, q: &SearchQuery) -> Vec<ComponentSummary> }
```

## Source
Lines 622–688 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| calls | [split_readable_docs](/crates/oxide-library/src/search_index/split_readable_docs.md) |
