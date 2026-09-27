---
okf_version: "0.2"
type: Module
title: selection
description: Footprint editor — selection update logic.
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection
language: rust
---

# selection

Footprint editor — selection update logic.

## Docstring

Footprint editor — selection update logic.

Split out of `apply_footprint_primitive_edit` per ADR-0001 D1/D2.
`apply` is a thin router; each `FootprintEditorMsg` variant delegates
to one named per-action fn below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |
| related | [select_active_idx](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_active_idx.md) |
| related | [toggle_selection_filter](/crates/oxide-app/src/library/editor/footprint/updates/selection/toggle_selection_filter.md) |
| related | [select_silk_f](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_silk_f.md) |
| related | [delete_silk_f](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_silk_f.md) |
| related | [move_pad](/crates/oxide-app/src/library/editor/footprint/updates/selection/move_pad.md) |
| related | [cursor_at](/crates/oxide-app/src/library/editor/footprint/updates/selection/cursor_at.md) |
| related | [select_pad](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_pad.md) |
| related | [select_pads](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_pads.md) |
| related | [delete_selected](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_selected.md) |
| related | [set_selection_mode_2d](/crates/oxide-app/src/library/editor/footprint/updates/selection/set_selection_mode_2d.md) |
| related | [select_all_on_layer](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_all_on_layer.md) |
| related | [lasso_arm](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_arm.md) |
| related | [lasso_add_vertex](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_add_vertex.md) |
| related | [lasso_cancel](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_cancel.md) |
| related | [lasso_commit](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_commit.md) |
| related | [touching_line_arm](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_arm.md) |
| related | [touching_line_first](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_first.md) |
| related | [touching_line_cancel](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_cancel.md) |
| related | [touching_line_commit](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_commit.md) |
| related | [select_overlapped](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_overlapped.md) |
| related | [select_off_grid_pads](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_off_grid_pads.md) |
