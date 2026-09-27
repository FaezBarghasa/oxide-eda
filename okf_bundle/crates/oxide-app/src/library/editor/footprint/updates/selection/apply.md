---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/apply
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
Lines 11–42 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| calls | [select_active_idx](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_active_idx.md) |
| calls | [toggle_selection_filter](/crates/oxide-app/src/library/editor/footprint/updates/selection/toggle_selection_filter.md) |
| calls | [select_silk_f](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_silk_f.md) |
| calls | [delete_silk_f](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_silk_f.md) |
| calls | [move_pad](/crates/oxide-app/src/library/editor/footprint/updates/selection/move_pad.md) |
| calls | [cursor_at](/crates/oxide-app/src/library/editor/footprint/updates/selection/cursor_at.md) |
| calls | [select_pad](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_pad.md) |
| calls | [select_pads](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_pads.md) |
| calls | [delete_selected](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_selected.md) |
| calls | [set_selection_mode_2d](/crates/oxide-app/src/library/editor/footprint/updates/selection/set_selection_mode_2d.md) |
| calls | [select_all_on_layer](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_all_on_layer.md) |
| calls | [lasso_arm](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_arm.md) |
| calls | [lasso_add_vertex](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_add_vertex.md) |
| calls | [lasso_cancel](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_cancel.md) |
| calls | [lasso_commit](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_commit.md) |
| calls | [touching_line_arm](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_arm.md) |
| calls | [touching_line_first](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_first.md) |
| calls | [touching_line_cancel](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_cancel.md) |
| calls | [touching_line_commit](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_commit.md) |
| calls | [select_overlapped](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_overlapped.md) |
| calls | [select_off_grid_pads](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_off_grid_pads.md) |
