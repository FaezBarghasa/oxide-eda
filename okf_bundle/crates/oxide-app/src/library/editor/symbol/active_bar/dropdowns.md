---
okf_version: "0.2"
type: Module
title: dropdowns
description: v0.13 — SchLib (.snxsym) editor active-bar dropdown menu
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns
language: rust
---

# dropdowns

v0.13 — SchLib (.snxsym) editor active-bar dropdown menu

## Docstring

v0.13 — SchLib (.snxsym) editor active-bar dropdown menu
definitions.

Each `SymActiveBarMenu` variant maps to a function that returns a
list of `DropdownEntry<LibraryMessage>` rows. Rendering lives in
`oxide_widgets::active_bar_dropdown::view`; the chevron-trigger
buttons + overlay positioning are owned by `symbol/active_bar.rs`.

Wiring philosophy mirrors the footprint editor: items that map to
existing primitives (Selection Filter pills, Shape tools) emit the
real `SymbolEditorMsg`; items that need new primitives emit
`SymbolActiveBarStub` so the action logs a "coming soon" warn and
dismisses the menu cleanly.

## Relationships

| Type | Target |
|------|--------|
| related | [sym](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/sym.md) |
| related | [stub](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/stub.md) |
| related | [stub_with_icon](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/stub_with_icon.md) |
| related | [entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/entries.md) |
| related | [filter_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/filter_entries.md) |
| related | [snap_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/snap_entries.md) |
| related | [place_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/place_entries.md) |
| related | [select_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/select_entries.md) |
| related | [align_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/align_entries.md) |
| related | [pin_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/pin_entries.md) |
| related | [text_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/text_entries.md) |
| related | [shapes_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/shapes_entries.md) |
| related | [item_msg](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/item_msg.md) |
| related | [place_move_row_arms_select_tool](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/place_move_row_arms_select_tool.md) |
| related | [align_to_grid_row_snaps_selection](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/align_to_grid_row_snaps_selection.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
