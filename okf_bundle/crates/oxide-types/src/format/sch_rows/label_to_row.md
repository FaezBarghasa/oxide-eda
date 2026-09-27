---
okf_version: "0.2"
type: Function
title: label_to_row
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows/label_to_row
language: rust
---

# label_to_row

## Signature

```rust
pub(in crate::format) fn label_to_row(l: &Label) -> SchLabelRow
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 384–397 in `crates/oxide-types/src/format/sch_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_rows](/crates/oxide-types/src/format/sch_rows.md) |
| calls | [mm_to_nm](/crates/oxide-types/src/format/units/mm_to_nm.md) |
| calls | [label_kind_str](/crates/oxide-types/src/format/sch_rows/label_kind_str.md) |
| calls | [halign_str](/crates/oxide-types/src/format/sch_rows/halign_str.md) |
| calls | [valign_str](/crates/oxide-types/src/format/sch_rows/valign_str.md) |
