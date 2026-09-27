---
okf_version: "0.2"
type: Function
title: list_rows_in_table
description: "Read every row in `table_name` for `library_id`, ordered by"
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/list_rows_in_table_1
language: rust
---

# list_rows_in_table

Read every row in `table_name` for `library_id`, ordered by

## Signature

```rust
pub fn list_rows_in_table(
        &self,
        library_id: Uuid,
        table_name: &str,
    ) -> Result<Vec<ComponentRow>, ApiError>
```

## Visibility

- `pub`

## Docstring

Read every row in `table_name` for `library_id`, ordered by
`internal_pn`.

## Source
Lines 289–310 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
