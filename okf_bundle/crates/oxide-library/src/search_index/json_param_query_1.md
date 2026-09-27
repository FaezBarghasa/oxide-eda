---
okf_version: "0.2"
type: Function
title: json_param_query
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/json_param_query_1
language: rust
---

# json_param_query

## Signature

```rust
fn json_param_query(
        &self,
        key: &str,
        facet: &Facet,
    ) -> Result<Box<dyn Query>, TantivyIndexError>
```

## Source
Lines 555–591 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
