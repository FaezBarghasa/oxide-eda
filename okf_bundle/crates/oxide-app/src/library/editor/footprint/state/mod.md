---
okf_version: "0.2"
type: Module
title: state
description: Footprint editor in-memory state.
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod
language: rust
---

# state

Footprint editor in-memory state.

## Docstring

Footprint editor in-memory state.

The canvas state derives from a typed
[`oxide_library::Footprint`] primitive — pad geometry mirrors
`Footprint::pads: Vec<Pad>`. Two-way sync runs through
[`FootprintEditorState::sync_pads_to_primitive`] so the dispatcher
keeps the primitive authoritative.

Dispatcher convention: every mutating op edits the canvas state,
then calls `sync_pads_to_primitive(&canvas_state, &mut footprint)`
to write the new pad list back onto the primitive.

Submodules:
- [`pad`] — `EditorPad`, `PadStackUi`, `NextPadDefaults`, `PadSide`,
`CourtyardRect`, `ShapeParamMap`, `AlignOp`.
- [`mode`] — `EditorMode`, `FpActiveBarMenu`, `PadStackTab`.
- [`context_menu`] — right-click menu state types.
- [`placement`] — `PlacementInput*`, `PlaceArcPending`.
- [`selection`] — `selected_pad_indices`, the shared selection union.
- [`tool`] — `PadsTool`, `SketchTool`, `ToolPending`.
- [`selection_filter`] — `SelectionFilter`, `SelectionFilterKind`.
- [`snap_options`] — `SnapOptions`, `GridDef`, `Guide*`, `SnapSubTab`,
`SnappingMode`, `GridDisplay`.

## Relationships

| Type | Target |
|------|--------|
| related | [MoveByModal](/crates/oxide-app/src/library/editor/footprint/state/mod/MoveByModal.md) |
| related | [parsed](/crates/oxide-app/src/library/editor/footprint/state/mod/parsed.md) |
| related | [parsed](/crates/oxide-app/src/library/editor/footprint/state/mod/parsed.md) |
| related | [AlignModal](/crates/oxide-app/src/library/editor/footprint/state/mod/AlignModal.md) |
| related | [FootprintEditorState](/crates/oxide-app/src/library/editor/footprint/state/mod/FootprintEditorState.md) |
| related | [from_footprint](/crates/oxide-app/src/library/editor/footprint/state/mod/from_footprint.md) |
| related | [empty](/crates/oxide-app/src/library/editor/footprint/state/mod/empty.md) |
| related | [with_pads](/crates/oxide-app/src/library/editor/footprint/state/mod/with_pads.md) |
| related | [content_bbox_mm](/crates/oxide-app/src/library/editor/footprint/state/mod/content_bbox_mm.md) |
| related | [next_pad_number](/crates/oxide-app/src/library/editor/footprint/state/mod/next_pad_number.md) |
| related | [add_pad_at](/crates/oxide-app/src/library/editor/footprint/state/mod/add_pad_at.md) |
| related | [add_hole_at](/crates/oxide-app/src/library/editor/footprint/state/mod/add_hole_at.md) |
| related | [move_pad](/crates/oxide-app/src/library/editor/footprint/state/mod/move_pad.md) |
| related | [nudge_pads](/crates/oxide-app/src/library/editor/footprint/state/mod/nudge_pads.md) |
| related | [delete_pad](/crates/oxide-app/src/library/editor/footprint/state/mod/delete_pad.md) |
| related | [pad_at](/crates/oxide-app/src/library/editor/footprint/state/mod/pad_at.md) |
| related | [recompute_courtyard](/crates/oxide-app/src/library/editor/footprint/state/mod/recompute_courtyard.md) |
| related | [toggle_auto_fit](/crates/oxide-app/src/library/editor/footprint/state/mod/toggle_auto_fit.md) |
| related | [recompute_courtyard_outline](/crates/oxide-app/src/library/editor/footprint/state/mod/recompute_courtyard_outline.md) |
| related | [refresh_pads_from_primitive](/crates/oxide-app/src/library/editor/footprint/state/mod/refresh_pads_from_primitive.md) |
| related | [sync_pads_to_primitive](/crates/oxide-app/src/library/editor/footprint/state/mod/sync_pads_to_primitive.md) |
| related | [from_footprint](/crates/oxide-app/src/library/editor/footprint/state/mod/from_footprint.md) |
| related | [empty](/crates/oxide-app/src/library/editor/footprint/state/mod/empty.md) |
| related | [with_pads](/crates/oxide-app/src/library/editor/footprint/state/mod/with_pads.md) |
| related | [content_bbox_mm](/crates/oxide-app/src/library/editor/footprint/state/mod/content_bbox_mm.md) |
| related | [next_pad_number](/crates/oxide-app/src/library/editor/footprint/state/mod/next_pad_number.md) |
| related | [add_pad_at](/crates/oxide-app/src/library/editor/footprint/state/mod/add_pad_at.md) |
| related | [add_hole_at](/crates/oxide-app/src/library/editor/footprint/state/mod/add_hole_at.md) |
| related | [move_pad](/crates/oxide-app/src/library/editor/footprint/state/mod/move_pad.md) |
| related | [nudge_pads](/crates/oxide-app/src/library/editor/footprint/state/mod/nudge_pads.md) |
| related | [delete_pad](/crates/oxide-app/src/library/editor/footprint/state/mod/delete_pad.md) |
| related | [pad_at](/crates/oxide-app/src/library/editor/footprint/state/mod/pad_at.md) |
| related | [recompute_courtyard](/crates/oxide-app/src/library/editor/footprint/state/mod/recompute_courtyard.md) |
| related | [toggle_auto_fit](/crates/oxide-app/src/library/editor/footprint/state/mod/toggle_auto_fit.md) |
| related | [recompute_courtyard_outline](/crates/oxide-app/src/library/editor/footprint/state/mod/recompute_courtyard_outline.md) |
| related | [refresh_pads_from_primitive](/crates/oxide-app/src/library/editor/footprint/state/mod/refresh_pads_from_primitive.md) |
| related | [sync_pads_to_primitive](/crates/oxide-app/src/library/editor/footprint/state/mod/sync_pads_to_primitive.md) |
| related | [adjust_selection_after_remove](/crates/oxide-app/src/library/editor/footprint/state/mod/adjust_selection_after_remove.md) |
| related | [from_footprint_round_trips_pads](/crates/oxide-app/src/library/editor/footprint/state/mod/from_footprint_round_trips_pads.md) |
| related | [add_pad_assigns_next_number](/crates/oxide-app/src/library/editor/footprint/state/mod/add_pad_assigns_next_number.md) |
| related | [sync_pads_to_primitive_writes_back](/crates/oxide-app/src/library/editor/footprint/state/mod/sync_pads_to_primitive_writes_back.md) |
| related | [refresh_relinks_pad_from_sketch_pad_attr](/crates/oxide-app/src/library/editor/footprint/state/mod/refresh_relinks_pad_from_sketch_pad_attr.md) |
