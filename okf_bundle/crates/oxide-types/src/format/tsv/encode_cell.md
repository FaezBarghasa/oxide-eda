---
okf_version: "0.2"
type: Function
title: encode_cell
description: "Encode a single TSV cell. Empty strings emit `\"\"` so column"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/encode_cell
language: rust
---

# encode_cell

Encode a single TSV cell. Empty strings emit `""` so column

## Signature

```rust
pub(in crate::format) fn encode_cell(cell: &str) -> String
```

## Visibility

- `pub(in crate::format)`

## Docstring

Encode a single TSV cell. Empty strings emit `""` so column
boundaries stay legible when split on whitespace.

A cell is quoted when it contains whitespace, a `"`, a `\`, or is
the literal `-` (which `decode_cell` maps to empty per the format
spec — without quoting, a real `-` value round-trips to `""`).
Inside a quoted cell, backslash and the control characters that
would otherwise break TSV row / whitespace splitting are escaped
with backslash sequences, and inner quotes are doubled (CSV style,
preserved for backward compatibility with existing files).

## Source
Lines 26–49 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| called_by | [encode_decode_cell_round_trips_dangerous_characters](/crates/oxide-types/src/format/tests/encode_decode_cell_round_trips_dangerous_characters.md) |
| called_by | [write_tsv_block](/crates/oxide-types/src/format/tsv/write_tsv_block.md) |
