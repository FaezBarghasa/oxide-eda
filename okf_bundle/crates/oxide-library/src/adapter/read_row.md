---
okf_version: "0.2"
type: Function
title: read_row
description: ── Row CRUD ────────────────────────────────────────────────────────
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/read_row
language: rust
---

# read_row

── Row CRUD ────────────────────────────────────────────────────────

## Signature

```rust
fn read_row(&self, _table: &str, _row_id: RowId) -> Result<ComponentRow, LibraryError>
```

## Docstring

── Row CRUD ────────────────────────────────────────────────────────

## Source
Lines 362–366 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
