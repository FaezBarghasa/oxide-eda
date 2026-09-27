---
okf_version: "0.2"
type: Function
title: doc_to_summary
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/doc_to_summary
language: rust
---

# doc_to_summary

## Signature

```rust
impl TantivySearchIndex { fn doc_to_summary(&self, doc: &TantivyDocument) -> Option<ComponentSummary> }
```

## Source
Lines 597–618 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| calls | [read_text](/crates/oxide-library/src/search_index/read_text.md) |
| calls | [parse_lifecycle_token](/crates/oxide-library/src/search_index/parse_lifecycle_token.md) |
