---
okf_version: "0.2"
type: Function
title: write_string
description: Serialise to a TOML+TSV string for writing to disk.
resource: crates/oxide-types/src/format/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/format/mod/write_string_3
language: rust
---

# write_string

Serialise to a TOML+TSV string for writing to disk.

## Signature

```rust
pub fn write_string(&self) -> Result<String, FormatError>
```

## Visibility

- `pub`

## Docstring

Serialise to a TOML+TSV string for writing to disk.

## Source
Lines 416–542 in `crates/oxide-types/src/format/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [format](/crates/oxide-types/src/format/mod.md) |
| calls | [pad_to_row](/crates/oxide-types/src/format/pcb_rows/pad_to_row.md) |
| calls | [write_tsv_section](/crates/oxide-types/src/format/tsv/write_tsv_section.md) |
