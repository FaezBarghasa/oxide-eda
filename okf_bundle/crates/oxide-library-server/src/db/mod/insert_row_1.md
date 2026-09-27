---
okf_version: "0.2"
type: Function
title: insert_row
description: "Insert a brand-new row. Returns `Ok(false)` when a row with the"
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/insert_row_1
language: rust
---

# insert_row

Insert a brand-new row. Returns `Ok(false)` when a row with the

## Signature

```rust
pub fn insert_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row: &ComponentRow,
    ) -> Result<bool, ApiError>
```

## Visibility

- `pub`

## Docstring

Insert a brand-new row. Returns `Ok(false)` when a row with the
same `(library_id, table, row_id)` already exists so the caller
can answer `409` — POST must never silently overwrite an
existing row. Replacement goes through [`update_row`] (PUT).

## Source
Lines 144–186 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
