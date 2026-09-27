---
okf_version: "0.2"
type: Function
title: editor_state_proj
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/editor_state_proj
language: rust
---

# editor_state_proj

## Signature

```rust
fn editor_state_proj(app: &Oxide, path: &std::path::Path) -> EditorStateProj
```

## Source
Lines 93–107 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| called_by | [ctrl_z_during_tangent_arc_undoes_last_segment](/crates/oxide-app/tests/regression/library_cross_track/ctrl_z_during_tangent_arc_undoes_last_segment.md) |
| called_by | [place_round_rect_then_select_arc_unlink_then_undo_restores_link](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_select_arc_unlink_then_undo_restores_link.md) |
| called_by | [place_round_rect_then_undo_restores_pre_place_state](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_undo_restores_pre_place_state.md) |
| called_by | [placement_input_does_not_corrupt_history_on_undo](/crates/oxide-app/tests/regression/library_cross_track/placement_input_does_not_corrupt_history_on_undo.md) |
