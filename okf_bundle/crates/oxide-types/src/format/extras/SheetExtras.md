---
okf_version: "0.2"
type: Class
title: SheetExtras
description: "Fields on [`SchematicSheet`] that aren't yet TSV-tabularised"
resource: crates/oxide-types/src/format/extras.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/extras/SheetExtras
language: rust
---

# SheetExtras

Fields on [`SchematicSheet`] that aren't yet TSV-tabularised

## Signature

```rust
pub(in crate::format) struct SheetExtras
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub(in crate::format)`

## Docstring

Fields on [`SchematicSheet`] that aren't yet TSV-tabularised
(rare in real designs, hierarchical or schema-rich): hierarchical
child sheets, no-connect markers, text notes, buses, bus entries,
drawing primitives, no-ERC directives, title block, and library
symbol cache.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `child_sheets`
- `no_connects`
- `text_notes`
- `buses`
- `bus_entries`
- `drawings`
- `no_erc_directives`
- `title_block`
- `lib_symbols`

## Source
Lines 158–177 in `crates/oxide-types/src/format/extras.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [extras](/crates/oxide-types/src/format/extras.md) |
