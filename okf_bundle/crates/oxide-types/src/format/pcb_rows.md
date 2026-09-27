---
okf_version: "0.2"
type: Module
title: pcb_rows
description: "PCB bulk-row DTOs (`[footprints]` / `[pads]` / `[tracks]` /"
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows
language: rust
---

# pcb_rows

PCB bulk-row DTOs (`[footprints]` / `[pads]` / `[tracks]` /

## Docstring

PCB bulk-row DTOs (`[footprints]` / `[pads]` / `[tracks]` /
`[vias]` TSV blocks) and their model translation.

Holds the flat row schemas — [`PcbFootprintRow`], [`PcbPadRow`],
[`PcbTrackRow`], [`PcbViaRow`] — plus their [`SnxTable`] impls, the
PCB-side enum string codecs, and the `Footprint` / `Pad` /
`Segment` / `Via` ↔ row translation. Pure code motion out of
`mod.rs`; the row types stay `pub` (public surface via the `format`
re-exports), the translation helpers are `pub(in crate::format)` so
the container `SnxPcb` can reach them.

## Relationships

| Type | Target |
|------|--------|
| related | [PcbFootprintRow](/crates/oxide-types/src/format/pcb_rows/PcbFootprintRow.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [PcbPadRow](/crates/oxide-types/src/format/pcb_rows/PcbPadRow.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [PcbTrackRow](/crates/oxide-types/src/format/pcb_rows/PcbTrackRow.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [PcbViaRow](/crates/oxide-types/src/format/pcb_rows/PcbViaRow.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [columns](/crates/oxide-types/src/format/pcb_rows/columns.md) |
| related | [to_row](/crates/oxide-types/src/format/pcb_rows/to_row.md) |
| related | [from_row](/crates/oxide-types/src/format/pcb_rows/from_row.md) |
| related | [pad_type_str](/crates/oxide-types/src/format/pcb_rows/pad_type_str.md) |
| related | [parse_pad_type](/crates/oxide-types/src/format/pcb_rows/parse_pad_type.md) |
| related | [pad_shape_str](/crates/oxide-types/src/format/pcb_rows/pad_shape_str.md) |
| related | [parse_pad_shape](/crates/oxide-types/src/format/pcb_rows/parse_pad_shape.md) |
| related | [via_type_str](/crates/oxide-types/src/format/pcb_rows/via_type_str.md) |
| related | [parse_via_type](/crates/oxide-types/src/format/pcb_rows/parse_via_type.md) |
| related | [join_layers](/crates/oxide-types/src/format/pcb_rows/join_layers.md) |
| related | [split_layers](/crates/oxide-types/src/format/pcb_rows/split_layers.md) |
| related | [footprint_to_row](/crates/oxide-types/src/format/pcb_rows/footprint_to_row.md) |
| related | [row_to_footprint](/crates/oxide-types/src/format/pcb_rows/row_to_footprint.md) |
| related | [pad_to_row](/crates/oxide-types/src/format/pcb_rows/pad_to_row.md) |
| related | [row_to_pad](/crates/oxide-types/src/format/pcb_rows/row_to_pad.md) |
| related | [track_to_row](/crates/oxide-types/src/format/pcb_rows/track_to_row.md) |
| related | [row_to_track](/crates/oxide-types/src/format/pcb_rows/row_to_track.md) |
| related | [via_to_row](/crates/oxide-types/src/format/pcb_rows/via_to_row.md) |
| related | [row_to_via](/crates/oxide-types/src/format/pcb_rows/row_to_via.md) |
