---
okf_version: "0.2"
type: Function
title: place_round_rect_then_undo_restores_pre_place_state
description: "Phase-5 #1 — `FootprintAddPad` placing a RoundRect pad runs"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_undo_restores_pre_place_state
language: rust
---

# place_round_rect_then_undo_restores_pre_place_state

Phase-5 #1 — `FootprintAddPad` placing a RoundRect pad runs

## Signature

```rust
fn place_round_rect_then_undo_restores_pre_place_state()
```

## Decorators

- `test`

## Docstring

Phase-5 #1 — `FootprintAddPad` placing a RoundRect pad runs
through `apply_footprint_primitive_edit`'s
`mutates_footprint_state` gate, which classifies it as mutating
state and calls `push_history()` first. A subsequent
`Message::Edit(EditMsg::Undo)` must restore the pre-place projection
(one fewer pad + the parametric geometry the mirror minted gone).
`Message::Edit(EditMsg::Redo)` must roll forward to the post-place
projection again.
[test]

## Source
Lines 142–197 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [fixture_empty_footprint_editor](/crates/oxide-app/tests/regression/library_cross_track/fixture_empty_footprint_editor.md) |
| calls | [set_pad_defaults](/crates/oxide-app/tests/regression/library_cross_track/set_pad_defaults.md) |
| calls | [editor_state_proj](/crates/oxide-app/tests/regression/library_cross_track/editor_state_proj.md) |
