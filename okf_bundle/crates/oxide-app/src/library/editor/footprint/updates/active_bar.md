---
okf_version: "0.2"
type: Module
title: active_bar
description: Footprint editor — active_bar update logic.
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar
language: rust
---

# active_bar

Footprint editor — active_bar update logic.

## Docstring

Footprint editor — active_bar update logic.

Split out of `apply_footprint_primitive_edit` per ADR-0001 D1/D2.
`apply` is a thin router; each `FootprintEditorMsg` variant delegates
to one named per-action fn below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |
| related | [toggle_active_bar_menu](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/toggle_active_bar_menu.md) |
| related | [close_active_bar_menu](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/close_active_bar_menu.md) |
| related | [active_bar_stub](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_stub.md) |
| related | [apply_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply_filter_preset.md) |
| related | [toggle_all_filters](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/toggle_all_filters.md) |
| related | [capture_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/capture_filter_preset.md) |
| related | [active_bar_toggle_snap](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_toggle_snap.md) |
| related | [active_bar_set_snapping_mode](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_set_snapping_mode.md) |
| related | [active_bar_set_snap_sub_tab](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_set_snap_sub_tab.md) |
| related | [active_bar_rotate_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_rotate_selection.md) |
| related | [active_bar_flip_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_flip_selection.md) |
| related | [active_bar_nudge_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_nudge_selection.md) |
| related | [move_by_open](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_open.md) |
| related | [move_by_set_x](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_set_x.md) |
| related | [move_by_set_y](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_set_y.md) |
| related | [move_by_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_confirm.md) |
| related | [move_by_cancel](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_cancel.md) |
| related | [align_open](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_open.md) |
| related | [align_set_horizontal](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_set_horizontal.md) |
| related | [align_set_vertical](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_set_vertical.md) |
| related | [align_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_confirm.md) |
| related | [align_cancel](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_cancel.md) |
| related | [active_bar_align_selection_to_grid](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_align_selection_to_grid.md) |
| related | [active_bar_move_origin_to_grid](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_move_origin_to_grid.md) |
| related | [active_bar_select_all](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_select_all.md) |
| related | [active_bar_clear_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_clear_selection.md) |
| related | [active_bar_set_sketch_tool](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_set_sketch_tool.md) |
| related | [flip_layer](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/flip_layer.md) |
