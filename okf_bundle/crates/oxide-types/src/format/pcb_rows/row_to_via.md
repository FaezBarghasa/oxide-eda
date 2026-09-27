---
okf_version: "0.2"
type: Function
title: row_to_via
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/row_to_via
language: rust
---

# row_to_via

## Signature

```rust
pub(in crate::format) fn row_to_via(row: PcbViaRow) -> Via
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 491–505 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
| calls | [nm_to_mm](/crates/oxide-types/src/format/units/nm_to_mm.md) |
| calls | [split_layers](/crates/oxide-types/src/format/pcb_rows/split_layers.md) |
| calls | [parse_via_type](/crates/oxide-types/src/format/pcb_rows/parse_via_type.md) |
