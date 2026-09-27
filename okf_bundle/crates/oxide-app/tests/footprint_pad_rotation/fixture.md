---
okf_version: "0.2"
type: Function
title: fixture
description: "Fresh app + a footprint editor holding `count` default pads, with"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/fixture
language: rust
---

# fixture

Fresh app + a footprint editor holding `count` default pads, with

## Signature

```rust
fn fixture(stem: &str, count: usize) -> (Oxide, PathBuf, TempDir)
```

## Docstring

Fresh app + a footprint editor holding `count` default pads, with
the active tab pointed at it so `Message::Edit(EditMsg::Undo)`
resolves through `active_footprint_editor_path()`.

## Source
Lines 30–61 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| called_by | [flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad](/crates/oxide-app/tests/footprint_pad_rotation/flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad.md) |
| called_by | [flip_moves_every_selected_pad_to_the_back_side](/crates/oxide-app/tests/footprint_pad_rotation/flip_moves_every_selected_pad_to_the_back_side.md) |
| called_by | [one_undo_reverses_the_whole_multi_pad_rotate](/crates/oxide-app/tests/footprint_pad_rotation/one_undo_reverses_the_whole_multi_pad_rotate.md) |
| called_by | [rotate_turns_every_pad_in_the_selection](/crates/oxide-app/tests/footprint_pad_rotation/rotate_turns_every_pad_in_the_selection.md) |
| called_by | [touching_line_scores_the_rotated_pad_not_the_unrotated_box](/crates/oxide-app/tests/footprint_pad_rotation/touching_line_scores_the_rotated_pad_not_the_unrotated_box.md) |
