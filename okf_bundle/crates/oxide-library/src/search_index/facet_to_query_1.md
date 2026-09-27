---
okf_version: "0.2"
type: Function
title: facet_to_query
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/facet_to_query_1
language: rust
---

# facet_to_query

## Signature

```rust
fn facet_to_query(&self, facet: &Facet) -> Result<Box<dyn Query>, TantivyIndexError>
```

## Source
Lines 453–471 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
