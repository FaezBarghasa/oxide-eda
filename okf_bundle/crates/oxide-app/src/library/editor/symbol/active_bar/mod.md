---
okf_version: "0.2"
type: Module
title: active_bar
description: "SchLib editor's Active Bar — the floating tool bar over the"
resource: crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/mod
language: rust
---

# active_bar

SchLib editor's Active Bar — the floating tool bar over the

## Docstring

SchLib editor's Active Bar — the floating tool bar over the
`.snxsym` canvas, mirroring the schematic editor's Altium-style
Active Bar pattern but with SchLib-specific tools.

v0.13 — Eight Altium dropdown menus (Filter / Snap / Place /
Select / Align / Pin / Text / Shapes) live at the FRONT of the
bar; their bodies come from `active_bar_dropdowns::entries`. The
Select / Place Pin tool slots stay at the end of the bar for
quick keyboard / single-click access. Pure-graphics tools (Line /
Arc / Circle / Rectangle) live in the Shapes dropdown to keep the
bar slim.

Built on top of the unified
`oxide_widgets::active_bar::view_with_overlay` so a single call
returns the bar + dropdown overlay + click-outside backstop —
identical pattern across schematic / footprint / SchLib /
upcoming PCB editors.

## Relationships

| Type | Target |
|------|--------|
| related | [bar_items](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/bar_items.md) |
| related | [dropdown_overlay](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/dropdown_overlay.md) |
| related | [dropdown_trigger_items](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/dropdown_trigger_items.md) |
| related | [new_editor](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/new_editor.md) |
| related | [left_click_symbol_msg](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/left_click_symbol_msg.md) |
| related | [move_trigger_left_click_arms_select_tool](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/move_trigger_left_click_arms_select_tool.md) |
| related | [align_trigger_left_click_snaps_selection_to_grid](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/align_trigger_left_click_snaps_selection_to_grid.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
