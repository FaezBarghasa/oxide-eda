---
okf_version: "0.2"
type: Function
title: parse_i64
description: "---------------------------------------------------------------------------"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/parse_i64
language: rust
---

# parse_i64

---------------------------------------------------------------------------

## Signature

```rust
pub(in crate::format) fn parse_i64(
    value: &str,
    block: &str,
    row: usize,
    field: &str,
) -> Result<i64, FormatError>
```

## Visibility

- `pub(in crate::format)`

## Docstring

---------------------------------------------------------------------------
Field-level parse helpers
---------------------------------------------------------------------------

## Source
Lines 283–297 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| called_by | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| called_by | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
