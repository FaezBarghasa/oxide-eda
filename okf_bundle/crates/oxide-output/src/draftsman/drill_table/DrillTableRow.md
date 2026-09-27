---
okf_version: "0.2"
type: Class
title: DrillTableRow
description: A row in the fabrication drill table.
resource: crates/oxide-output/src/draftsman/drill_table.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:25:04Z"
concept_id: crates/oxide-output/src/draftsman/drill_table/DrillTableRow
language: rust
---

# DrillTableRow

A row in the fabrication drill table.

## Signature

```rust
pub struct DrillTableRow
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

A row in the fabrication drill table.
[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `symbol_char`
- `diameter_nm`
- `diameter_mm`
- `diameter_mil`
- `plating`
- `count`
- `tolerance_plus_mm`
- `tolerance_minus_mm`
- `layer_pair`

## Source
Lines 20–30 in `crates/oxide-output/src/draftsman/drill_table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drill_table](/crates/oxide-output/src/draftsman/drill_table.md) |
