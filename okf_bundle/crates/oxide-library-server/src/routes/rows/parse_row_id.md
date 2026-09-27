---
okf_version: "0.2"
type: Function
title: parse_row_id
resource: crates/oxide-library-server/src/routes/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:11Z"
concept_id: crates/oxide-library-server/src/routes/rows/parse_row_id
language: rust
---

# parse_row_id

## Signature

```rust
fn parse_row_id(raw: &str) -> Result<RowId, ApiError>
```

## Source
Lines 112–115 in `crates/oxide-library-server/src/routes/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-library-server/src/routes/rows.md) |
| called_by | [delete_row](/crates/oxide-library-server/src/routes/rows/delete_row.md) |
| called_by | [get_row](/crates/oxide-library-server/src/routes/rows/get_row.md) |
| called_by | [update_row](/crates/oxide-library-server/src/routes/rows/update_row.md) |
