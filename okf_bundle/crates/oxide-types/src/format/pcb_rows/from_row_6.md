---
okf_version: "0.2"
type: Function
title: from_row
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/from_row_6
language: rust
---

# from_row

## Signature

```rust
impl PcbViaRow { fn from_row(values: &[&str], block: &str, row: usize) -> Result<Self, FormatError> }
```

## Source
Lines 242–260 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
| calls | [parse_uuid](/crates/oxide-types/src/format/tsv/parse_uuid.md) |
| calls | [parse_i64](/crates/oxide-types/src/format/tsv/parse_i64.md) |
