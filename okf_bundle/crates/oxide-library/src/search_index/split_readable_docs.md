---
okf_version: "0.2"
type: Function
title: split_readable_docs
description: Split a batch of stored-document reads into the documents that came
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/split_readable_docs
language: rust
---

# split_readable_docs

Split a batch of stored-document reads into the documents that came

## Signature

```rust
fn split_readable_docs(reads: I) -> (Vec<TantivyDocument>, UnreadableHits)
```

## Type Parameters

- `I`

## Docstring

Split a batch of stored-document reads into the documents that came
back and a tally of the ones that failed.

## Source
Lines 704–722 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| called_by | [a_clean_batch_keeps_every_document_and_tallies_nothing](/crates/oxide-library/src/search_index/a_clean_batch_keeps_every_document_and_tallies_nothing.md) |
| called_by | [query](/crates/oxide-library/src/search_index/query.md) |
| called_by | [unreadable_documents_are_counted_and_the_first_error_survives](/crates/oxide-library/src/search_index/unreadable_documents_are_counted_and_the_first_error_survives.md) |
