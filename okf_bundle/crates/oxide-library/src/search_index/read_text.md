---
okf_version: "0.2"
type: Function
title: read_text
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/read_text
language: rust
---

# read_text

## Signature

```rust
fn read_text(doc: &TantivyDocument, field: Field) -> Option<String>
```

## Source
Lines 781–785 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| called_by | [doc_to_summary](/crates/oxide-library/src/search_index/doc_to_summary.md) |
| called_by | [main](/installer/build-wordmark/main.md) |
| called_by | [main](/tools/regen_wordmark_path/main.md) |
| called_by | [main](/tools/sync_wordmark_from_black/main.md) |
