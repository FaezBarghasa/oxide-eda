---
okf_version: "0.2"
type: Function
title: ingest_row
description: "Replace the `primitive_to_rows` entries for a single row (idempotent)."
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/ingest_row_1
language: rust
---

# ingest_row

Replace the `primitive_to_rows` entries for a single row (idempotent).

## Signature

```rust
pub fn ingest_row(&mut self, row: &ComponentRow)
```

## Visibility

- `pub`

## Docstring

Replace the `primitive_to_rows` entries for a single row (idempotent).

Used by adapters when a single row is saved or updated mid-session,
so we don't have to re-scan the whole library.

## Source
Lines 138–146 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
