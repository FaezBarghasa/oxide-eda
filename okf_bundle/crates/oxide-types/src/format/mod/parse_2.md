---
okf_version: "0.2"
type: Function
title: parse
description: Parse a TOML+TSV string from disk.
resource: crates/oxide-types/src/format/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/format/mod/parse_2
language: rust
---

# parse

Parse a TOML+TSV string from disk.

## Signature

```rust
impl SnxPcb { pub fn parse(input: &str) -> Result<Self, FormatError> }
```

## Visibility

- `pub`

## Docstring

Parse a TOML+TSV string from disk.

## Source
Lines 545–662 in `crates/oxide-types/src/format/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [format](/crates/oxide-types/src/format/mod.md) |
| calls | [row_to_footprint](/crates/oxide-types/src/format/pcb_rows/row_to_footprint.md) |
| calls | [row_to_pad](/crates/oxide-types/src/format/pcb_rows/row_to_pad.md) |
