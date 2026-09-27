# input

## Classs

- [Claim](Claim.md) — One consumer's answer for one event.
- [InputConsumer](InputConsumer.md) — The app-level keyboard consumers, as identities rather than as
- [InputTarget](InputTarget.md) — Where an input event landed, and therefore what the user can see

## Functions

- [a_detached_preferences_moves_the_recorder_with_it](a_detached_preferences_moves_the_recorder_with_it.md) — [test]
- [a_non_digit_command_chord_falls_through_to_the_keymap](a_non_digit_command_chord_falls_through_to_the_keymap.md) — #127 — without the `selection_slot_from_key` guard these arms
- [an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in](an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in.md) — The headline symptom. With the palette open, an Esc typed inside a
- [an_open_palette_stops_swallowing_keys_in_windows_it_does_not_paint_in](an_open_palette_stops_swallowing_keys_in_windows_it_does_not_paint_in.md) — [test]
- [an_ordinary_stroke_reaches_the_keymap_resolver](an_ordinary_stroke_reaches_the_keymap_resolver.md) — [test]
- [character](character.md)
- [claim](claim.md) — What `consumer` makes of this event.
- [claim](claim_1.md) — What `consumer` makes of this event.
- [claim_command_palette](claim_command_palette.md) — Command palette. Captures most input while open so typing into
- [claim_command_palette](claim_command_palette_1.md) — Command palette. Captures most input while open so typing into
- [claim_escape](claim_escape.md) — Esc is forwarded raw and resolved in `dispatch/escape.rs` against
- [claim_escape](claim_escape_1.md) — Esc is forwarded raw and resolved in `dispatch/escape.rs` against
- [claim_keymap](claim_keymap.md) — Everything else routes through the active keymap profile: forward
- [claim_keymap](claim_keymap_1.md) — Everything else routes through the active keymap profile: forward
- [claim_keymap_recorder](claim_keymap_recorder.md) — Chord recorder (Preferences ▸ Keyboard Shortcuts). Exclusive while
- [claim_keymap_recorder](claim_keymap_recorder_1.md) — Chord recorder (Preferences ▸ Keyboard Shortcuts). Exclusive while
- [claim_order_lists_every_consumer_exactly_once](claim_order_lists_every_consumer_exactly_once.md) — [test]
- [claim_selection_slots](claim_selection_slots.md) — Ctrl+1-8 store selection memory, Alt+1-8 recall it. These carry
- [claim_selection_slots](claim_selection_slots_1.md) — Ctrl+1-8 store selection memory, Alt+1-8 recall it. These carry
- [claim_shortcuts_sheet](claim_shortcuts_sheet.md) — F1 toggles the keyboard-shortcuts sheet: open if closed, close if
- [claim_shortcuts_sheet](claim_shortcuts_sheet_1.md) — F1 toggles the keyboard-shortcuts sheet: open if closed, close if
- [command_and_alt_digits_carry_the_slot_as_data](command_and_alt_digits_carry_the_slot_as_data.md) — [test]
- [digits_without_ctrl_or_alt_fall_through_to_the_keymap](digits_without_ctrl_or_alt_fall_through_to_the_keymap.md) — [test]
- [escape_is_forwarded_raw_with_its_window](escape_is_forwarded_raw_with_its_window.md) — [test]
- [events_nobody_wants_produce_no_message](events_nobody_wants_produce_no_message.md) — [test]
- [f1_is_a_dead_key_where_the_sheet_cannot_appear](f1_is_a_dead_key_where_the_sheet_cannot_appear.md) — [test]
- [f1_toggles_the_shortcuts_sheet_both_ways](f1_toggles_the_shortcuts_sheet_both_ways.md) — [test]
- [handle_key_input](handle_key_input.md) — The single entry point for `Message::KeyInput`.
- [handle_key_input](handle_key_input_1.md) — The single entry point for `Message::KeyInput`.
- [input_target](input_target.md) — Classify the window an input event landed in.
- [input_target](input_target_1.md) — Classify the window an input event landed in.
- [input_target_classifies_every_window_kind](input_target_classifies_every_window_kind.md) — [test]
- [main_window](main_window.md)
- [named](named.md)
- [only_main_and_undocked_paint_the_overlay_stack](only_main_and_undocked_paint_the_overlay_stack.md) — [test]
- [open_window](open_window.md) — Give `kind` its own OS window and hand back that window's id.
- [paints_overlay_stack](paints_overlay_stack.md) — Whether this window paints the main overlay stack.
- [paints_overlay_stack](paints_overlay_stack_1.md) — Whether this window paints the main overlay stack.
- [press](press.md)
- [quiet_app](quiet_app.md)
- [recorder_visible_in](recorder_visible_in.md) — Whether the chord recorder is on screen in `target`.
- [recorder_visible_in](recorder_visible_in_1.md) — Whether the chord recorder is on screen in `target`.
- [recording](recording.md) — A recorder mid-capture. `KeymapRecorderState` has no `Default` —
- [released](released.md)
- [route_key](route_key.md) — Walk [`CLAIM_ORDER`] and return the one message this event earns,
- [route_key](route_key_1.md) — Walk [`CLAIM_ORDER`] and return the one message this event earns,
- [selection_slot_from_key](selection_slot_from_key.md) — Which selection-memory slot a digit key names, if any.
- [selection_slot_only_matches_digits_one_through_eight](selection_slot_only_matches_digits_one_through_eight.md) — Moved here with the helper. The `None` half is a regression
- [the_palette_leaks_only_its_four_navigation_keys](the_palette_leaks_only_its_four_navigation_keys.md) — [test]
- [the_palette_still_owns_its_keys_where_it_does_paint](the_palette_still_owns_its_keys_where_it_does_paint.md) — [test]
- [the_palette_swallows_tool_shortcuts_so_typing_does_not_fire_them](the_palette_swallows_tool_shortcuts_so_typing_does_not_fire_them.md) — [test]
- [the_recorder_claims_every_stroke_while_open](the_recorder_claims_every_stroke_while_open.md) — [test]
- [the_recorder_claims_only_where_preferences_is_painted_inline](the_recorder_claims_only_where_preferences_is_painted_inline.md) — [test]
- [the_recorder_outranks_the_palette](the_recorder_outranks_the_palette.md) — [test]
- [the_recorder_swallows_what_it_cannot_express](the_recorder_swallows_what_it_cannot_express.md) — [test]
- [unidentified](unidentified.md)
