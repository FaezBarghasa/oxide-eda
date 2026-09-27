---
okf_version: "0.2"
type: Module
title: format
description: "Oxide native file formats — `.snxsch` (schematic) and `.snxpcb` (PCB)."
resource: crates/oxide-types/src/format/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/format/mod
language: rust
---

# format

Oxide native file formats — `.snxsch` (schematic) and `.snxpcb` (PCB).

## Docstring

Oxide native file formats — `.snxsch` (schematic) and `.snxpcb` (PCB).

Wire format: TOML envelope + TSV bulk-block pattern (matches
`.snxlib` / `.snxsym` / `.snxfpt` from the v0.9 library refactor).
The first lines of every file are a TOML manifest (`format`, IDs).
For each bulk entity type, a single TOML table emits a `content`
key whose value is a literal multi-line TSV string — the first row
is the column header, subsequent rows are data, columns are
whitespace-separated. Hierarchical or rare-field data (zone
polygons, the stackup, custom properties) lives in regular TOML
sub-tables alongside the TSV blocks.

`.snxprj` (project) is unchanged and uses its own pre-existing
format. Stays as-is.

These types are the canonical Oxide schema. Standard I/O — when it
returns via the `oxide-standard-import` companion repo (GPL-3.0) —
translates to/from these types at the file-format boundary; no
Standard-shaped types live in this Apache codebase.

## Relationships

| Type | Target |
|------|--------|
| related | [FormatError](/crates/oxide-types/src/format/mod/FormatError.md) |
| related | [SnxTable](/crates/oxide-types/src/format/mod/SnxTable.md) |
| related | [SnxSchematic](/crates/oxide-types/src/format/mod/SnxSchematic.md) |
| related | [new](/crates/oxide-types/src/format/mod/new.md) |
| related | [write_string](/crates/oxide-types/src/format/mod/write_string.md) |
| related | [parse](/crates/oxide-types/src/format/mod/parse.md) |
| related | [new](/crates/oxide-types/src/format/mod/new.md) |
| related | [write_string](/crates/oxide-types/src/format/mod/write_string.md) |
| related | [ExtrasWrapper](/crates/oxide-types/src/format/mod/ExtrasWrapper.md) |
| related | [ExtrasInner](/crates/oxide-types/src/format/mod/ExtrasInner.md) |
| related | [parse](/crates/oxide-types/src/format/mod/parse.md) |
| related | [SchManifest](/crates/oxide-types/src/format/mod/SchManifest.md) |
| related | [TsvBody](/crates/oxide-types/src/format/mod/TsvBody.md) |
| related | [SchSheetsRaw](/crates/oxide-types/src/format/mod/SchSheetsRaw.md) |
| related | [SchRaw](/crates/oxide-types/src/format/mod/SchRaw.md) |
| related | [SnxPcb](/crates/oxide-types/src/format/mod/SnxPcb.md) |
| related | [new](/crates/oxide-types/src/format/mod/new.md) |
| related | [write_string](/crates/oxide-types/src/format/mod/write_string.md) |
| related | [parse](/crates/oxide-types/src/format/mod/parse.md) |
| related | [new](/crates/oxide-types/src/format/mod/new.md) |
| related | [write_string](/crates/oxide-types/src/format/mod/write_string.md) |
| related | [StackupWrapper](/crates/oxide-types/src/format/mod/StackupWrapper.md) |
| related | [StackBlock](/crates/oxide-types/src/format/mod/StackBlock.md) |
| related | [NetsBlock](/crates/oxide-types/src/format/mod/NetsBlock.md) |
| related | [ZonesWrapper](/crates/oxide-types/src/format/mod/ZonesWrapper.md) |
| related | [ExtrasWrapper](/crates/oxide-types/src/format/mod/ExtrasWrapper.md) |
| related | [ExtrasInner](/crates/oxide-types/src/format/mod/ExtrasInner.md) |
| related | [parse](/crates/oxide-types/src/format/mod/parse.md) |
| related | [PcbManifest](/crates/oxide-types/src/format/mod/PcbManifest.md) |
| related | [PcbRaw](/crates/oxide-types/src/format/mod/PcbRaw.md) |
| related | [StackRaw](/crates/oxide-types/src/format/mod/StackRaw.md) |
| related | [NetsRaw](/crates/oxide-types/src/format/mod/NetsRaw.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
