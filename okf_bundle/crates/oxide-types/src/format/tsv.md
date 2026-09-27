---
okf_version: "0.2"
type: Module
title: tsv
description: "TSV bulk-block codec: cell encode/decode, TOML-envelope escaping,"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv
language: rust
---

# tsv

TSV bulk-block codec: cell encode/decode, TOML-envelope escaping,

## Docstring

TSV bulk-block codec: cell encode/decode, TOML-envelope escaping,
row splitting, the block reader/writer, the field-level parse
helpers, and the numeric formatter.

Pure code motion out of `mod.rs`. The cross-module helpers are
`pub(in crate::format)` (visible to the whole `format` module tree,
exactly as when they lived in the single file); the block
reader/writer stay `pub` (part of the crate's public surface via
the `format` re-exports). CODE MOTION ONLY — the escaping and
quoting rules are byte-for-byte unchanged (a wrong byte here makes a
schematic unreloadable, #96).

## Relationships

| Type | Target |
|------|--------|
| related | [encode_cell](/crates/oxide-types/src/format/tsv/encode_cell.md) |
| related | [decode_cell](/crates/oxide-types/src/format/tsv/decode_cell.md) |
| related | [escape_tsv_body_for_toml](/crates/oxide-types/src/format/tsv/escape_tsv_body_for_toml.md) |
| related | [split_row](/crates/oxide-types/src/format/tsv/split_row.md) |
| related | [write_tsv_block](/crates/oxide-types/src/format/tsv/write_tsv_block.md) |
| related | [parse_tsv_block](/crates/oxide-types/src/format/tsv/parse_tsv_block.md) |
| related | [parse_i64](/crates/oxide-types/src/format/tsv/parse_i64.md) |
| related | [parse_f64](/crates/oxide-types/src/format/tsv/parse_f64.md) |
| related | [parse_uuid](/crates/oxide-types/src/format/tsv/parse_uuid.md) |
| related | [format_f64](/crates/oxide-types/src/format/tsv/format_f64.md) |
| related | [write_tsv_section](/crates/oxide-types/src/format/tsv/write_tsv_section.md) |
