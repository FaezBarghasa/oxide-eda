---
okf_version: "0.2"
type: Function
title: pads_to_tsv
description: "Encode a slice of pads as TSV — header row first, then one row"
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/pads_to_tsv
language: rust
---

# pads_to_tsv

Encode a slice of pads as TSV — header row first, then one row

## Signature

```rust
pub(crate) fn pads_to_tsv(pads: &[Pad]) -> Result<String, FootprintFileError>
```

## Visibility

- `pub(crate)`

## Docstring

Encode a slice of pads as TSV — header row first, then one row
per pad. Empty slice still emits the header row.

## Source
Lines 226–235 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| calls | [pad_to_tsv_row](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_to_tsv_row.md) |
| called_by | [to_toml_string](/crates/oxide-library/src/primitive/footprint/mod/to_toml_string.md) |
| called_by | [pads_to_tsv_empty_emits_header_only](/crates/oxide-library/src/primitive/footprint/tests/pads_to_tsv_empty_emits_header_only.md) |
| called_by | [pads_to_tsv_rejects_tab_in_cell](/crates/oxide-library/src/primitive/footprint/tests/pads_to_tsv_rejects_tab_in_cell.md) |
