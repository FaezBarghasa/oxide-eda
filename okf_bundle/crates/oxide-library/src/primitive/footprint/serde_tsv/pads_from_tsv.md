---
okf_version: "0.2"
type: Function
title: pads_from_tsv
description: "Parse a `pads_tsv` payload back into `Vec<Pad>`. The first non-"
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/pads_from_tsv
language: rust
---

# pads_from_tsv

Parse a `pads_tsv` payload back into `Vec<Pad>`. The first non-

## Signature

```rust
pub(crate) fn pads_from_tsv(tsv: &str) -> Result<Vec<Pad>, FootprintFileError>
```

## Visibility

- `pub(crate)`

## Docstring

Parse a `pads_tsv` payload back into `Vec<Pad>`. The first non-
empty line is the header and must equal [`PAD_TSV_COLUMNS`]; each
subsequent line is a pad row.

## Source
Lines 240–271 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| calls | [pad_from_tsv_row](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_from_tsv_row.md) |
| called_by | [from_toml_str](/crates/oxide-library/src/primitive/footprint/mod/from_toml_str.md) |
| called_by | [pads_from_tsv_rejects_drill_slot_without_diameter](/crates/oxide-library/src/primitive/footprint/tests/pads_from_tsv_rejects_drill_slot_without_diameter.md) |
| called_by | [pads_from_tsv_rejects_schema_mismatch](/crates/oxide-library/src/primitive/footprint/tests/pads_from_tsv_rejects_schema_mismatch.md) |
