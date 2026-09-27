---
okf_version: "0.2"
type: Function
title: from_row
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows/from_row_4
language: rust
---

# from_row

## Signature

```rust
impl SchJunctionRow { fn from_row(values: &[&str], block: &str, row: usize) -> Result<Self, FormatError> }
```

## Source
Lines 146–153 in `crates/oxide-types/src/format/sch_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_rows](/crates/oxide-types/src/format/sch_rows.md) |
| calls | [parse_uuid](/crates/oxide-types/src/format/tsv/parse_uuid.md) |
| calls | [parse_i64](/crates/oxide-types/src/format/tsv/parse_i64.md) |
| calls | [parse_f64](/crates/oxide-types/src/format/tsv/parse_f64.md) |
