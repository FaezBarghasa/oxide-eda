---
okf_version: "0.2"
type: Module
title: extras
description: "Auxiliary \"extras\" sub-tables — the per-symbol / per-sheet and"
resource: crates/oxide-types/src/format/extras.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/extras
language: rust
---

# extras

Auxiliary "extras" sub-tables — the per-symbol / per-sheet and

## Docstring

Auxiliary "extras" sub-tables — the per-symbol / per-sheet and
per-footprint / per-pad / per-board fields that don't fit a flat
TSV row, plus the raw deserialization envelopes for the `[extras]`
TOML tree.

Pure code motion out of `mod.rs`. Every type, field, and method is
`pub(in crate::format)` — visible to the whole `format` module tree
exactly as when it was module-private in the single file — so the
container types in `mod.rs` and the row-translation helpers in
`sch_rows` / `pcb_rows` can reach them. The `#[serde(...)]`
attributes and the `default_*` functions are unchanged (they define
the on-disk defaults; a changed default silently rewrites data).

## Relationships

| Type | Target |
|------|--------|
| related | [SchExtrasRaw](/crates/oxide-types/src/format/extras/SchExtrasRaw.md) |
| related | [JunctionExtras](/crates/oxide-types/src/format/extras/JunctionExtras.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_junction](/crates/oxide-types/src/format/extras/from_junction.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_junction](/crates/oxide-types/src/format/extras/from_junction.md) |
| related | [SymbolExtras](/crates/oxide-types/src/format/extras/SymbolExtras.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_symbol](/crates/oxide-types/src/format/extras/from_symbol.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_symbol](/crates/oxide-types/src/format/extras/from_symbol.md) |
| related | [SheetExtras](/crates/oxide-types/src/format/extras/SheetExtras.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_sheet](/crates/oxide-types/src/format/extras/from_sheet.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_sheet](/crates/oxide-types/src/format/extras/from_sheet.md) |
| related | [default_true](/crates/oxide-types/src/format/extras/default_true.md) |
| related | [default_unit](/crates/oxide-types/src/format/extras/default_unit.md) |
| related | [PcbExtrasRaw](/crates/oxide-types/src/format/extras/PcbExtrasRaw.md) |
| related | [PcbExtras](/crates/oxide-types/src/format/extras/PcbExtras.md) |
| related | [from_board](/crates/oxide-types/src/format/extras/from_board.md) |
| related | [from_board](/crates/oxide-types/src/format/extras/from_board.md) |
| related | [FootprintExtras](/crates/oxide-types/src/format/extras/FootprintExtras.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_footprint](/crates/oxide-types/src/format/extras/from_footprint.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_footprint](/crates/oxide-types/src/format/extras/from_footprint.md) |
| related | [PadExtras](/crates/oxide-types/src/format/extras/PadExtras.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_pad](/crates/oxide-types/src/format/extras/from_pad.md) |
| related | [is_default](/crates/oxide-types/src/format/extras/is_default.md) |
| related | [from_pad](/crates/oxide-types/src/format/extras/from_pad.md) |
| related | [BoardExtras](/crates/oxide-types/src/format/extras/BoardExtras.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
