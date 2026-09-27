---
okf_version: "0.2"
type: Function
title: main_window
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/main_window
language: rust
---

# main_window

## Signature

```rust
fn main_window(app: &Oxide) -> iced::window::Id
```

## Source
Lines 471–475 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| called_by | [a_detached_preferences_moves_the_recorder_with_it](/crates/oxide-app/src/app/dispatch/input/a_detached_preferences_moves_the_recorder_with_it.md) |
| called_by | [a_non_digit_command_chord_falls_through_to_the_keymap](/crates/oxide-app/src/app/dispatch/input/a_non_digit_command_chord_falls_through_to_the_keymap.md) |
| called_by | [an_ordinary_stroke_reaches_the_keymap_resolver](/crates/oxide-app/src/app/dispatch/input/an_ordinary_stroke_reaches_the_keymap_resolver.md) |
| called_by | [command_and_alt_digits_carry_the_slot_as_data](/crates/oxide-app/src/app/dispatch/input/command_and_alt_digits_carry_the_slot_as_data.md) |
| called_by | [digits_without_ctrl_or_alt_fall_through_to_the_keymap](/crates/oxide-app/src/app/dispatch/input/digits_without_ctrl_or_alt_fall_through_to_the_keymap.md) |
| called_by | [escape_is_forwarded_raw_with_its_window](/crates/oxide-app/src/app/dispatch/input/escape_is_forwarded_raw_with_its_window.md) |
| called_by | [events_nobody_wants_produce_no_message](/crates/oxide-app/src/app/dispatch/input/events_nobody_wants_produce_no_message.md) |
| called_by | [f1_toggles_the_shortcuts_sheet_both_ways](/crates/oxide-app/src/app/dispatch/input/f1_toggles_the_shortcuts_sheet_both_ways.md) |
| called_by | [the_palette_leaks_only_its_four_navigation_keys](/crates/oxide-app/src/app/dispatch/input/the_palette_leaks_only_its_four_navigation_keys.md) |
| called_by | [the_palette_swallows_tool_shortcuts_so_typing_does_not_fire_them](/crates/oxide-app/src/app/dispatch/input/the_palette_swallows_tool_shortcuts_so_typing_does_not_fire_them.md) |
| called_by | [the_recorder_claims_every_stroke_while_open](/crates/oxide-app/src/app/dispatch/input/the_recorder_claims_every_stroke_while_open.md) |
| called_by | [the_recorder_claims_only_where_preferences_is_painted_inline](/crates/oxide-app/src/app/dispatch/input/the_recorder_claims_only_where_preferences_is_painted_inline.md) |
| called_by | [the_recorder_outranks_the_palette](/crates/oxide-app/src/app/dispatch/input/the_recorder_outranks_the_palette.md) |
| called_by | [the_recorder_swallows_what_it_cannot_express](/crates/oxide-app/src/app/dispatch/input/the_recorder_swallows_what_it_cannot_express.md) |
