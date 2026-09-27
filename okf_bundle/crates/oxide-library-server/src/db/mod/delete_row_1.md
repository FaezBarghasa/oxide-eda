---
okf_version: "0.2"
type: Function
title: delete_row
description: "Delete a row. Returns `Ok(false)` if no matching row existed."
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/delete_row_1
language: rust
---

# delete_row

Delete a row. Returns `Ok(false)` if no matching row existed.

## Signature

```rust
pub fn delete_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row_id: RowId,
    ) -> Result<bool, ApiError>
```

## Visibility

- `pub`

## Docstring

Delete a row. Returns `Ok(false)` if no matching row existed.

## Source
Lines 245–263 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
