---
okf_version: "0.2"
type: Function
title: parse_lifecycle_token
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/parse_lifecycle_token
language: rust
---

# parse_lifecycle_token

## Signature

```rust
fn parse_lifecycle_token(s: &str) -> Option<LifecycleState>
```

## Source
Lines 770–779 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| called_by | [doc_to_summary](/crates/oxide-library/src/search_index/doc_to_summary.md) |
