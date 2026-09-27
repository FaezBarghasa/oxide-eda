---
okf_version: "0.2"
type: Function
title: parse_oxide_markup
description: Parse Oxide markup into a flat list of rich segments.
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/parse_oxide_markup
language: rust
---

# parse_oxide_markup

Parse Oxide markup into a flat list of rich segments.

## Signature

```rust
pub fn parse_oxide_markup(input: &str) -> Vec<RichSegment>
```

## Visibility

- `pub`

## Docstring

Parse Oxide markup into a flat list of rich segments.

Sigils are consumed in order; the parser is single-pass and does not
handle nested formatting (e.g. `**_~OE~_**` produces a Bold segment
containing the literal text `_~OE~_`). Use whichever decoration
matters most semantically.

## Source
Lines 198–317 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| calls | [read_overbar](/crates/oxide-types/src/markup/read_overbar.md) |
| calls | [flush_normal](/crates/oxide-types/src/markup/flush_normal.md) |
| calls | [read_paired_double](/crates/oxide-types/src/markup/read_paired_double.md) |
| calls | [read_paired_single](/crates/oxide-types/src/markup/read_paired_single.md) |
| calls | [read_paired_bracket](/crates/oxide-types/src/markup/read_paired_bracket.md) |
| calls | [read_paired_paren](/crates/oxide-types/src/markup/read_paired_paren.md) |
| called_by | [pdf_markup_runs](/crates/oxide-output/src/pdf/content/pdf_markup_runs.md) |
| called_by | [schematic_text_offset_hier](/crates/oxide-output/src/svg/labels/schematic_text_offset_hier.md) |
| called_by | [markup_runs](/crates/oxide-output/src/svg/text/markup_runs.md) |
