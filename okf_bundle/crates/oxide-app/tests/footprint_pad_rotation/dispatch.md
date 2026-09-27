---
okf_version: "0.2"
type: Function
title: dispatch
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/dispatch
language: rust
---

# dispatch

## Signature

```rust
fn dispatch(app: &mut Oxide, path: &Path, msg: FootprintEditorMsg)
```

## Source
Lines 63–68 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| called_by | [flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad](/crates/oxide-app/tests/footprint_pad_rotation/flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad.md) |
| called_by | [flip_moves_every_selected_pad_to_the_back_side](/crates/oxide-app/tests/footprint_pad_rotation/flip_moves_every_selected_pad_to_the_back_side.md) |
| called_by | [flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper](/crates/oxide-app/tests/footprint_pad_rotation/flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper.md) |
| called_by | [one_undo_reverses_the_whole_multi_pad_rotate](/crates/oxide-app/tests/footprint_pad_rotation/one_undo_reverses_the_whole_multi_pad_rotate.md) |
| called_by | [rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper](/crates/oxide-app/tests/footprint_pad_rotation/rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper.md) |
| called_by | [rotate_turns_every_pad_in_the_selection](/crates/oxide-app/tests/footprint_pad_rotation/rotate_turns_every_pad_in_the_selection.md) |
| called_by | [touching_line_scores_the_rotated_pad_not_the_unrotated_box](/crates/oxide-app/tests/footprint_pad_rotation/touching_line_scores_the_rotated_pad_not_the_unrotated_box.md) |
