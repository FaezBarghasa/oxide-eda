---
okf_version: "0.2"
type: Class
title: SchematicSheet
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/schematic/sheet.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-types/src/schematic/sheet/SchematicSheet
language: rust
---

# SchematicSheet

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct SchematicSheet
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `uuid`
- `version`
- `generator`
- `generator_version`
- `paper_size`
- `root_sheet_page`
- `symbols`
- `wires`
- `junctions`
- `labels`
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
Lines 218–256 in `crates/oxide-types/src/schematic/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-types/src/schematic/sheet.md) |
