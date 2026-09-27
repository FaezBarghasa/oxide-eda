---
okf_version: "0.2"
type: Function
title: fixture_empty_footprint_editor
description: "Phase-5 helper — fresh `Oxide` + a `FootprintEditorState` parked"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/fixture_empty_footprint_editor
language: rust
---

# fixture_empty_footprint_editor

Phase-5 helper — fresh `Oxide` + a `FootprintEditorState` parked

## Signature

```rust
fn fixture_empty_footprint_editor(stem: &str) -> (Oxide, std::path::PathBuf, TempDir)
```

## Docstring

Phase-5 helper — fresh `Oxide` + a `FootprintEditorState` parked
in `document_state.footprint_editors` for a `<stem>.snxfpt` path
inside a tempdir. The active tab points at the editor with
`TabKind::FootprintEditor` so the `Message::Edit(EditMsg::Undo)`/`Redo` fork in
`handle_undo_requested` resolves the editor via
`active_footprint_editor_path()` and not the schematic engine.

The seeded sketch carries one placeholder Point so
`footprint_sketch_is_active` reports `true`; the dispatcher's
`FootprintAddPad` arm only mirrors a pad into the sketch when
the sketch already has at least one entity (avoids auto-minting
a sketch the user never visited).

## Source
Lines 33–79 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| called_by | [chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts](/crates/oxide-app/tests/regression/library_cross_track/chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts.md) |
| called_by | [place_round_rect_then_undo_restores_pre_place_state](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_undo_restores_pre_place_state.md) |
| called_by | [placement_input_does_not_corrupt_history_on_undo](/crates/oxide-app/tests/regression/library_cross_track/placement_input_does_not_corrupt_history_on_undo.md) |
| called_by | [tangent_arc_after_line_creates_tangent_constraint](/crates/oxide-app/tests/regression/library_cross_track/tangent_arc_after_line_creates_tangent_constraint.md) |
| called_by | [type_5_during_line_draw_commits_at_5mm](/crates/oxide-app/tests/regression/library_cross_track/type_5_during_line_draw_commits_at_5mm.md) |
