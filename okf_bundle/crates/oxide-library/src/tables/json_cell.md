---
okf_version: "0.2"
type: Function
title: json_cell
description: ── (de)serialisation helpers ─────────────────────────────────────────────
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/json_cell
language: rust
---

# json_cell

── (de)serialisation helpers ─────────────────────────────────────────────

## Signature

```rust
fn json_cell(value: &T) -> Result<String, LibraryError>
```

## Type Parameters

- `T: Serialize`

## Docstring

── (de)serialisation helpers ─────────────────────────────────────────────

## Source
Lines 214–216 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| called_by | [datasheet_to_cell](/crates/oxide-library/src/tables/datasheet_to_cell.md) |
| called_by | [opt_primitive_to_cell](/crates/oxide-library/src/tables/opt_primitive_to_cell.md) |
| called_by | [row_to_record](/crates/oxide-library/src/tables/row_to_record.md) |
