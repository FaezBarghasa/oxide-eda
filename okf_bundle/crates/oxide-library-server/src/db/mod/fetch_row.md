---
okf_version: "0.2"
type: Function
title: fetch_row
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/fetch_row
language: rust
---

# fetch_row

## Signature

```rust
impl AppState { pub fn fetch_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row_id: RowId,
    ) -> Result<Option<ComponentRow>, ApiError> }
```

## Visibility

- `pub`

## Source
Lines 220–242 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
