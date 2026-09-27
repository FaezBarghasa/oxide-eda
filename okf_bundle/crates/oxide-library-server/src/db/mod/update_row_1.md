---
okf_version: "0.2"
type: Function
title: update_row
description: "Update an existing row. Returns `Ok(false)` if no row with the"
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/update_row_1
language: rust
---

# update_row

Update an existing row. Returns `Ok(false)` if no row with the

## Signature

```rust
pub fn update_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row: &ComponentRow,
    ) -> Result<bool, ApiError>
```

## Visibility

- `pub`

## Docstring

Update an existing row. Returns `Ok(false)` if no row with the
supplied `(library_id, table, row_id)` exists; the caller maps that
to a 404.

## Source
Lines 191–218 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
