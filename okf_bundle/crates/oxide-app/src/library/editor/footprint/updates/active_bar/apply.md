---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply
language: rust
---

# apply

## Signature

```rust
pub(super) fn apply(editor: &mut crate::app::FootprintEditorState, msg: FootprintEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 13–50 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| calls | [toggle_active_bar_menu](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/toggle_active_bar_menu.md) |
| calls | [close_active_bar_menu](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/close_active_bar_menu.md) |
| calls | [active_bar_stub](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_stub.md) |
| calls | [apply_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply_filter_preset.md) |
| calls | [toggle_all_filters](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/toggle_all_filters.md) |
| calls | [capture_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/capture_filter_preset.md) |
| calls | [active_bar_toggle_snap](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_toggle_snap.md) |
| calls | [active_bar_set_snapping_mode](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_set_snapping_mode.md) |
| calls | [active_bar_set_snap_sub_tab](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_set_snap_sub_tab.md) |
| calls | [active_bar_rotate_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_rotate_selection.md) |
| calls | [active_bar_flip_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_flip_selection.md) |
| calls | [active_bar_nudge_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_nudge_selection.md) |
| calls | [move_by_open](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_open.md) |
| calls | [move_by_set_x](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_set_x.md) |
| calls | [move_by_set_y](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_set_y.md) |
| calls | [move_by_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_confirm.md) |
| calls | [move_by_cancel](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_cancel.md) |
| calls | [align_open](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_open.md) |
| calls | [align_set_horizontal](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_set_horizontal.md) |
| calls | [align_set_vertical](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_set_vertical.md) |
| calls | [align_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_confirm.md) |
| calls | [align_cancel](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_cancel.md) |
| calls | [active_bar_align_selection_to_grid](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_align_selection_to_grid.md) |
| calls | [active_bar_move_origin_to_grid](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_move_origin_to_grid.md) |
| calls | [active_bar_select_all](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_select_all.md) |
| calls | [active_bar_clear_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_clear_selection.md) |
| calls | [active_bar_set_sketch_tool](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_set_sketch_tool.md) |
