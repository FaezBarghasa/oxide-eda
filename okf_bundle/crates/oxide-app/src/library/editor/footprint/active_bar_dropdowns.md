---
okf_version: "0.2"
type: Module
title: active_bar_dropdowns
description: v0.13 — Footprint editor active-bar dropdown menu definitions.
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns
language: rust
---

# active_bar_dropdowns

v0.13 — Footprint editor active-bar dropdown menu definitions.

## Docstring

v0.13 — Footprint editor active-bar dropdown menu definitions.

Each `FpActiveBarMenu` variant maps to a function that returns the
list of `DropdownEntry<LibraryMessage>` rows. Rendering happens in
`oxide_widgets::active_bar_dropdown::view`; overlay positioning is
handled by the caller (`unified_active_bar`).

Wiring philosophy: every dropdown item here maps to an existing
primitive and emits the real `FootprintEditorMsg` (Selection Filter
pills, Snap toggles, snap-mode picks, Place tools, Drag Track End,
Break Track, Body3D, Extruded 3D Body, Move Selection by X,Y, the
Align… dialog, Text Frame). The [`stub`] helper + the
`FootprintEditorMsg::ActiveBarStub` "coming soon" variant are retained
(removing the variant is out of #372's scope) for any future
not-yet-implemented row, even though no current dropdown row uses them.

## Relationships

| Type | Target |
|------|--------|
| related | [fp](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/fp.md) |
| related | [stub](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/stub.md) |
| related | [align_item](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_item.md) |
| related | [align_item_with_icon](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_item_with_icon.md) |
| related | [entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/entries.md) |
| related | [sketch_create_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_create_entries.md) |
| related | [sketch_modify_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/sketch_modify_entries.md) |
| related | [filter_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/filter_entries.md) |
| related | [snap_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/snap_entries.md) |
| related | [place_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/place_entries.md) |
| related | [select_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/select_entries.md) |
| related | [align_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/align_entries.md) |
| related | [body3d_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/body3d_entries.md) |
| related | [text_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/text_entries.md) |
| related | [shapes_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/shapes_entries.md) |
| related | [shapes_dropdown_has_exactly_one_arc_row](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/shapes_dropdown_has_exactly_one_arc_row.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
