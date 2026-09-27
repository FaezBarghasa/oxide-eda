---
okf_version: "0.2"
type: Function
title: parse_f64
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/parse_f64
language: rust
---

# parse_f64

## Signature

```rust
fn parse_f64(facet: &Facet) -> Result<f64, TantivyIndexError>
```

## Source
Lines 749–758 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| called_by | [numeric_param_query](/crates/oxide-library/src/search_index/numeric_param_query.md) |
