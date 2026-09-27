---
okf_version: "0.2"
type: Module
title: sch_rows
description: "Schematic bulk-row DTOs (`[sheets.*]` TSV blocks) and their"
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows
language: rust
---

# sch_rows

Schematic bulk-row DTOs (`[sheets.*]` TSV blocks) and their

## Docstring

Schematic bulk-row DTOs (`[sheets.*]` TSV blocks) and their
model translation.

Holds the flat row schemas — [`SchComponentRow`], [`SchWireRow`],
[`SchJunctionRow`], [`SchLabelRow`] — plus their [`SnxTable`] impls,
the schematic-side enum string codecs, and the `Symbol` / `Wire` /
`Junction` / `Label` ↔ row translation. Pure code motion out of
`mod.rs`; the row types stay `pub` (public surface via the `format`
re-exports), the translation helpers are `pub(in crate::format)` so
the container `SnxSchematic` can reach them.

## Relationships

| Type | Target |
|------|--------|
| related | [SchComponentRow](/crates/oxide-types/src/format/sch_rows/SchComponentRow.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [SchWireRow](/crates/oxide-types/src/format/sch_rows/SchWireRow.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [SchJunctionRow](/crates/oxide-types/src/format/sch_rows/SchJunctionRow.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [SchLabelRow](/crates/oxide-types/src/format/sch_rows/SchLabelRow.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/sch_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/sch_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/sch_rows/from_row.md) |
| related | [label_kind_str](/crates/oxide-types/src/format/sch_rows/label_kind_str.md) |
| related | [parse_label_kind](/crates/oxide-types/src/format/sch_rows/parse_label_kind.md) |
| related | [halign_str](/crates/oxide-types/src/format/sch_rows/halign_str.md) |
| related | [parse_halign](/crates/oxide-types/src/format/sch_rows/parse_halign.md) |
| related | [valign_str](/crates/oxide-types/src/format/sch_rows/valign_str.md) |
| related | [parse_valign](/crates/oxide-types/src/format/sch_rows/parse_valign.md) |
| related | [symbol_to_row](/crates/oxide-types/src/format/sch_rows/symbol_to_row.md) |
| related | [row_to_symbol](/crates/oxide-types/src/format/sch_rows/row_to_symbol.md) |
| related | [wire_to_row](/crates/oxide-types/src/format/sch_rows/wire_to_row.md) |
| related | [row_to_wire](/crates/oxide-types/src/format/sch_rows/row_to_wire.md) |
| related | [junction_to_row](/crates/oxide-types/src/format/sch_rows/junction_to_row.md) |
| related | [row_to_junction](/crates/oxide-types/src/format/sch_rows/row_to_junction.md) |
| related | [label_to_row](/crates/oxide-types/src/format/sch_rows/label_to_row.md) |
| related | [row_to_label](/crates/oxide-types/src/format/sch_rows/row_to_label.md) |
