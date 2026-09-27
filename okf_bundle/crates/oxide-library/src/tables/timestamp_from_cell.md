---
okf_version: "0.2"
type: Function
title: timestamp_from_cell
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/timestamp_from_cell
language: rust
---

# timestamp_from_cell

## Signature

```rust
fn timestamp_from_cell(s: &str) -> Result<DateTime<Utc>, LibraryError>
```

## Source
Lines 277–281 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| called_by | [record_to_row](/crates/oxide-library/src/tables/record_to_row.md) |
