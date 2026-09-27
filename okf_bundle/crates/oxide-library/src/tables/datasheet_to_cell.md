---
okf_version: "0.2"
type: Function
title: datasheet_to_cell
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/datasheet_to_cell
language: rust
---

# datasheet_to_cell

## Signature

```rust
fn datasheet_to_cell(d: &DatasheetRef) -> Result<String, LibraryError>
```

## Source
Lines 222–224 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [json_cell](/crates/oxide-library/src/tables/json_cell.md) |
