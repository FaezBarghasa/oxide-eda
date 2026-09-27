---
okf_version: "0.2"
type: Function
title: open_window
description: "Give `kind` its own OS window and hand back that window's id."
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/open_window
language: rust
---

# open_window

Give `kind` its own OS window and hand back that window's id.

## Signature

```rust
fn open_window(app: &mut Oxide, kind: WindowKind) -> iced::window::Id
```

## Docstring

Give `kind` its own OS window and hand back that window's id.

## Source
Lines 482–486 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| called_by | [a_detached_preferences_moves_the_recorder_with_it](/crates/oxide-app/src/app/dispatch/input/a_detached_preferences_moves_the_recorder_with_it.md) |
| called_by | [an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in](/crates/oxide-app/src/app/dispatch/input/an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in.md) |
| called_by | [an_open_palette_stops_swallowing_keys_in_windows_it_does_not_paint_in](/crates/oxide-app/src/app/dispatch/input/an_open_palette_stops_swallowing_keys_in_windows_it_does_not_paint_in.md) |
| called_by | [f1_is_a_dead_key_where_the_sheet_cannot_appear](/crates/oxide-app/src/app/dispatch/input/f1_is_a_dead_key_where_the_sheet_cannot_appear.md) |
| called_by | [input_target_classifies_every_window_kind](/crates/oxide-app/src/app/dispatch/input/input_target_classifies_every_window_kind.md) |
| called_by | [the_palette_still_owns_its_keys_where_it_does_paint](/crates/oxide-app/src/app/dispatch/input/the_palette_still_owns_its_keys_where_it_does_paint.md) |
| called_by | [the_recorder_claims_only_where_preferences_is_painted_inline](/crates/oxide-app/src/app/dispatch/input/the_recorder_claims_only_where_preferences_is_painted_inline.md) |
