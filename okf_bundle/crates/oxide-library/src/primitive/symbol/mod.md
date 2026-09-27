---
okf_version: "0.2"
type: Module
title: symbol
description: "`Symbol` primitive — schematic-side reusable shape."
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod
language: rust
---

# symbol

`Symbol` primitive — schematic-side reusable shape.

## Docstring

`Symbol` primitive — schematic-side reusable shape.

Per `v0.9-refactor-2-plan.md` §2.1, a `Symbol` carries:
- typed pin list (no more opaque `(symbol …)` blob),
- drawing primitives (lines/rects/arcs/text),
- default schematic parameters that flow onto a binding `Component`.

## Relationships

| Type | Target |
|------|--------|
| related | [PinDirection](/crates/oxide-library/src/primitive/symbol/mod/PinDirection.md) |
| related | [PinOrientation](/crates/oxide-library/src/primitive/symbol/mod/PinOrientation.md) |
| related | [PinSymbolKind](/crates/oxide-library/src/primitive/symbol/mod/PinSymbolKind.md) |
| related | [SymbolPin](/crates/oxide-library/src/primitive/symbol/mod/SymbolPin.md) |
| related | [default_visibility_true](/crates/oxide-library/src/primitive/symbol/mod/default_visibility_true.md) |
| related | [default_part_number](/crates/oxide-library/src/primitive/symbol/mod/default_part_number.md) |
| related | [new](/crates/oxide-library/src/primitive/symbol/mod/new.md) |
| related | [new](/crates/oxide-library/src/primitive/symbol/mod/new.md) |
| related | [SymbolGraphicKind](/crates/oxide-library/src/primitive/symbol/mod/SymbolGraphicKind.md) |
| related | [normalize_arc_endpoints_deg](/crates/oxide-library/src/primitive/symbol/mod/normalize_arc_endpoints_deg.md) |
| related | [SymbolGraphic](/crates/oxide-library/src/primitive/symbol/mod/SymbolGraphic.md) |
| related | [ComponentType](/crates/oxide-library/src/primitive/symbol/mod/ComponentType.md) |
| related | [Symbol](/crates/oxide-library/src/primitive/symbol/mod/Symbol.md) |
| related | [default_version](/crates/oxide-library/src/primitive/symbol/mod/default_version.md) |
| related | [default_designator](/crates/oxide-library/src/primitive/symbol/mod/default_designator.md) |
| related | [default_comment](/crates/oxide-library/src/primitive/symbol/mod/default_comment.md) |
| related | [default_part_count](/crates/oxide-library/src/primitive/symbol/mod/default_part_count.md) |
| related | [empty](/crates/oxide-library/src/primitive/symbol/mod/empty.md) |
| related | [empty](/crates/oxide-library/src/primitive/symbol/mod/empty.md) |
| related | [SymbolFile](/crates/oxide-library/src/primitive/symbol/mod/SymbolFile.md) |
| related | [default_format](/crates/oxide-library/src/primitive/symbol/mod/default_format.md) |
| related | [SymbolFileWire](/crates/oxide-library/src/primitive/symbol/mod/SymbolFileWire.md) |
| related | [SymbolWire](/crates/oxide-library/src/primitive/symbol/mod/SymbolWire.md) |
| related | [migrate_legacy_arc](/crates/oxide-library/src/primitive/symbol/mod/migrate_legacy_arc.md) |
| related | [from_symbol](/crates/oxide-library/src/primitive/symbol/mod/from_symbol.md) |
| related | [get_symbol](/crates/oxide-library/src/primitive/symbol/mod/get_symbol.md) |
| related | [get_symbol_mut](/crates/oxide-library/src/primitive/symbol/mod/get_symbol_mut.md) |
| related | [upsert](/crates/oxide-library/src/primitive/symbol/mod/upsert.md) |
| related | [from_bytes](/crates/oxide-library/src/primitive/symbol/mod/from_bytes.md) |
| related | [from_toml_str](/crates/oxide-library/src/primitive/symbol/mod/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-library/src/primitive/symbol/mod/to_toml_string.md) |
| related | [from_symbol](/crates/oxide-library/src/primitive/symbol/mod/from_symbol.md) |
| related | [get_symbol](/crates/oxide-library/src/primitive/symbol/mod/get_symbol.md) |
| related | [get_symbol_mut](/crates/oxide-library/src/primitive/symbol/mod/get_symbol_mut.md) |
| related | [upsert](/crates/oxide-library/src/primitive/symbol/mod/upsert.md) |
| related | [from_bytes](/crates/oxide-library/src/primitive/symbol/mod/from_bytes.md) |
| related | [from_toml_str](/crates/oxide-library/src/primitive/symbol/mod/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-library/src/primitive/symbol/mod/to_toml_string.md) |
| related | [SymbolFileError](/crates/oxide-library/src/primitive/symbol/mod/SymbolFileError.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
