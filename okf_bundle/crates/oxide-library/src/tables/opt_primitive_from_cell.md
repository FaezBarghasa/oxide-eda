---
okf_version: "0.2"
type: Function
title: opt_primitive_from_cell
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/opt_primitive_from_cell
language: rust
---

# opt_primitive_from_cell

## Signature

```rust
fn opt_primitive_from_cell(s: &str) -> Result<Option<PrimitiveRef>, LibraryError>
```

## Source
Lines 265–271 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [from_json_cell](/crates/oxide-library/src/tables/from_json_cell.md) |
| called_by | [record_to_row](/crates/oxide-library/src/tables/record_to_row.md) |
