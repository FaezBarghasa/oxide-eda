---
okf_version: "0.2"
type: Function
title: junction_to_row
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows/junction_to_row
language: rust
---

# junction_to_row

## Signature

```rust
pub(in crate::format) fn junction_to_row(j: &Junction) -> SchJunctionRow
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 363–370 in `crates/oxide-types/src/format/sch_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_rows](/crates/oxide-types/src/format/sch_rows.md) |
| calls | [mm_to_nm](/crates/oxide-types/src/format/units/mm_to_nm.md) |
