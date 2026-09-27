---
okf_version: "0.2"
type: Function
title: iter_rows
description: "Iterate every row across every table — `(table_name, row)` pairs."
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/iter_rows
language: rust
---

# iter_rows

Iterate every row across every table — `(table_name, row)` pairs.

## Signature

```rust
fn iter_rows(&self) -> Result<Vec<(String, ComponentRow)>, LibraryError>
```

## Docstring

Iterate every row across every table — `(table_name, row)` pairs.
Used by `WhereUsedIndex::rebuild_from_rows` and the search index.

## Source
Lines 354–358 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
