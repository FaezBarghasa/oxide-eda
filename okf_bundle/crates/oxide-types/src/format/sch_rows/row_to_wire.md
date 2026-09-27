---
okf_version: "0.2"
type: Function
title: row_to_wire
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows/row_to_wire
language: rust
---

# row_to_wire

## Signature

```rust
pub(in crate::format) fn row_to_wire(row: SchWireRow) -> Wire
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 348–361 in `crates/oxide-types/src/format/sch_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_rows](/crates/oxide-types/src/format/sch_rows.md) |
| calls | [nm_to_mm](/crates/oxide-types/src/format/units/nm_to_mm.md) |
