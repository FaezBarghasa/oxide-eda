---
okf_version: "0.2"
type: Module
title: input
description: "Input routing: which app-level consumer owns a keyboard event, and"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input
language: rust
---

# input

Input routing: which app-level consumer owns a keyboard event, and

## Docstring

Input routing: which app-level consumer owns a keyboard event, and
in which window (#557 phase 1, #558).

# Two tiers, and this is the second

The keyboard subscription filters on `event::Status::Ignored`
(`app/bootstrap/subscription.rs`), and iced computes that status per
window from the widget tree. A widget that calls
`shell.capture_event()` makes the event invisible here. So **widgets
arbitrate first and this router only ever sees their leftovers.**

Widget capture is per-key, not per-widget, and that is load-bearing:
a focused `text_input` captures printable text, Backspace/Delete/
Home/End/ArrowLeft/ArrowRight, Ctrl+C/X/V/A — and Escape, which it
eats to unfocus itself (`iced_widget-0.14.2/src/text_input.rs`). It
does NOT capture F1, ArrowUp/ArrowDown, or unrecognised chords. The
command palette's arrow navigation works *only because* its focused
search field ignores those keys. No consumer below may therefore be
described as "sees every stroke"; the honest spec is "sees every
stroke no widget wanted". If the chord recorder's UI ever gains a
focused input, its capture semantics change silently — that is the
trap this paragraph exists to disarm.

The footprint canvas composes for free: its own key path returns
`.and_capture()` for the keys it owns
(`library/editor/footprint/canvas/input/keys.rs`), so those never
arrive, and the ones it declines do.

# Why the decision lives in `update`

It used to live in the subscription closure, deciding against a
`KeyContext` snapshot baked when the subscription was last rebuilt —
i.e. one update stale, and blind to which window produced the key.
#535 made that argument for Esc and moved it here; this module
finishes the job for the other consumers.

Deleting the snapshot also deletes churn: `Subscription::with` folds
its value into the subscription's identity hash, so every command
palette / recorder / shortcuts open-close used to tear down and
respawn the keyboard stream. The subscription's identity is now
constant for the process lifetime.

# Precedence is NOT paint order

The tempting move after #535 is "input precedence = reverse
`PAINT_ORDER`". It is wrong, and the counterexample is in the tree:
the chord recorder lives inside Preferences, which paints far BELOW
the command palette (`view/overlay_id.rs`), yet the recorder outranks
it for key claims — and must, or recording Ctrl+Shift+P into a
binding would be stolen by the palette it is trying to bind.

Input precedence is about capture *exclusivity*; paint order is about
*stacking*. They correlate; they are not the same fact. So
[`CLAIM_ORDER`] is its own contract, in `PAINT_ORDER`'s style but not
derived from it. Its payoff is smaller than `PAINT_ORDER`'s — there
is no second reader that must agree with the order — and the honest
reason it exists is the exhaustive match: a new consumer cannot
compile until it states its claim, which is what stops the next
capture being added as one more branch of an `if` chain.

# Not in scope

The mouse subscription is a second ad-hoc router (three
`listen().map` variants swapped on drag state, each with its own
identity churn). It is deliberately untouched — but [`InputTarget`]
and [`Claim`] are the shape it would migrate to, so please extend
this module rather than inventing a parallel one.

## Relationships

| Type | Target |
|------|--------|
| related | [InputTarget](/crates/oxide-app/src/app/dispatch/input/InputTarget.md) |
| related | [paints_overlay_stack](/crates/oxide-app/src/app/dispatch/input/paints_overlay_stack.md) |
| related | [paints_overlay_stack](/crates/oxide-app/src/app/dispatch/input/paints_overlay_stack.md) |
| related | [Claim](/crates/oxide-app/src/app/dispatch/input/Claim.md) |
| related | [InputConsumer](/crates/oxide-app/src/app/dispatch/input/InputConsumer.md) |
| related | [input_target](/crates/oxide-app/src/app/dispatch/input/input_target.md) |
| related | [handle_key_input](/crates/oxide-app/src/app/dispatch/input/handle_key_input.md) |
| related | [route_key](/crates/oxide-app/src/app/dispatch/input/route_key.md) |
| related | [claim](/crates/oxide-app/src/app/dispatch/input/claim.md) |
| related | [claim_keymap_recorder](/crates/oxide-app/src/app/dispatch/input/claim_keymap_recorder.md) |
| related | [recorder_visible_in](/crates/oxide-app/src/app/dispatch/input/recorder_visible_in.md) |
| related | [claim_command_palette](/crates/oxide-app/src/app/dispatch/input/claim_command_palette.md) |
| related | [claim_escape](/crates/oxide-app/src/app/dispatch/input/claim_escape.md) |
| related | [claim_shortcuts_sheet](/crates/oxide-app/src/app/dispatch/input/claim_shortcuts_sheet.md) |
| related | [claim_selection_slots](/crates/oxide-app/src/app/dispatch/input/claim_selection_slots.md) |
| related | [claim_keymap](/crates/oxide-app/src/app/dispatch/input/claim_keymap.md) |
| related | [input_target](/crates/oxide-app/src/app/dispatch/input/input_target.md) |
| related | [handle_key_input](/crates/oxide-app/src/app/dispatch/input/handle_key_input.md) |
| related | [route_key](/crates/oxide-app/src/app/dispatch/input/route_key.md) |
| related | [claim](/crates/oxide-app/src/app/dispatch/input/claim.md) |
| related | [claim_keymap_recorder](/crates/oxide-app/src/app/dispatch/input/claim_keymap_recorder.md) |
| related | [recorder_visible_in](/crates/oxide-app/src/app/dispatch/input/recorder_visible_in.md) |
| related | [claim_command_palette](/crates/oxide-app/src/app/dispatch/input/claim_command_palette.md) |
| related | [claim_escape](/crates/oxide-app/src/app/dispatch/input/claim_escape.md) |
| related | [claim_shortcuts_sheet](/crates/oxide-app/src/app/dispatch/input/claim_shortcuts_sheet.md) |
| related | [claim_selection_slots](/crates/oxide-app/src/app/dispatch/input/claim_selection_slots.md) |
| related | [claim_keymap](/crates/oxide-app/src/app/dispatch/input/claim_keymap.md) |
| related | [selection_slot_from_key](/crates/oxide-app/src/app/dispatch/input/selection_slot_from_key.md) |
| related | [quiet_app](/crates/oxide-app/src/app/dispatch/input/quiet_app.md) |
| related | [main_window](/crates/oxide-app/src/app/dispatch/input/main_window.md) |
| related | [unidentified](/crates/oxide-app/src/app/dispatch/input/unidentified.md) |
| related | [open_window](/crates/oxide-app/src/app/dispatch/input/open_window.md) |
| related | [press](/crates/oxide-app/src/app/dispatch/input/press.md) |
| related | [named](/crates/oxide-app/src/app/dispatch/input/named.md) |
| related | [character](/crates/oxide-app/src/app/dispatch/input/character.md) |
| related | [released](/crates/oxide-app/src/app/dispatch/input/released.md) |
| related | [recording](/crates/oxide-app/src/app/dispatch/input/recording.md) |
| related | [claim_order_lists_every_consumer_exactly_once](/crates/oxide-app/src/app/dispatch/input/claim_order_lists_every_consumer_exactly_once.md) |
| related | [input_target_classifies_every_window_kind](/crates/oxide-app/src/app/dispatch/input/input_target_classifies_every_window_kind.md) |
| related | [only_main_and_undocked_paint_the_overlay_stack](/crates/oxide-app/src/app/dispatch/input/only_main_and_undocked_paint_the_overlay_stack.md) |
| related | [the_recorder_claims_every_stroke_while_open](/crates/oxide-app/src/app/dispatch/input/the_recorder_claims_every_stroke_while_open.md) |
| related | [the_recorder_outranks_the_palette](/crates/oxide-app/src/app/dispatch/input/the_recorder_outranks_the_palette.md) |
| related | [the_recorder_swallows_what_it_cannot_express](/crates/oxide-app/src/app/dispatch/input/the_recorder_swallows_what_it_cannot_express.md) |
| related | [the_palette_leaks_only_its_four_navigation_keys](/crates/oxide-app/src/app/dispatch/input/the_palette_leaks_only_its_four_navigation_keys.md) |
| related | [the_palette_swallows_tool_shortcuts_so_typing_does_not_fire_them](/crates/oxide-app/src/app/dispatch/input/the_palette_swallows_tool_shortcuts_so_typing_does_not_fire_them.md) |
| related | [escape_is_forwarded_raw_with_its_window](/crates/oxide-app/src/app/dispatch/input/escape_is_forwarded_raw_with_its_window.md) |
| related | [f1_toggles_the_shortcuts_sheet_both_ways](/crates/oxide-app/src/app/dispatch/input/f1_toggles_the_shortcuts_sheet_both_ways.md) |
| related | [command_and_alt_digits_carry_the_slot_as_data](/crates/oxide-app/src/app/dispatch/input/command_and_alt_digits_carry_the_slot_as_data.md) |
| related | [a_non_digit_command_chord_falls_through_to_the_keymap](/crates/oxide-app/src/app/dispatch/input/a_non_digit_command_chord_falls_through_to_the_keymap.md) |
| related | [digits_without_ctrl_or_alt_fall_through_to_the_keymap](/crates/oxide-app/src/app/dispatch/input/digits_without_ctrl_or_alt_fall_through_to_the_keymap.md) |
| related | [an_ordinary_stroke_reaches_the_keymap_resolver](/crates/oxide-app/src/app/dispatch/input/an_ordinary_stroke_reaches_the_keymap_resolver.md) |
| related | [events_nobody_wants_produce_no_message](/crates/oxide-app/src/app/dispatch/input/events_nobody_wants_produce_no_message.md) |
| related | [an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in](/crates/oxide-app/src/app/dispatch/input/an_open_palette_no_longer_eats_escape_in_a_window_it_does_not_paint_in.md) |
| related | [an_open_palette_stops_swallowing_keys_in_windows_it_does_not_paint_in](/crates/oxide-app/src/app/dispatch/input/an_open_palette_stops_swallowing_keys_in_windows_it_does_not_paint_in.md) |
| related | [the_palette_still_owns_its_keys_where_it_does_paint](/crates/oxide-app/src/app/dispatch/input/the_palette_still_owns_its_keys_where_it_does_paint.md) |
| related | [f1_is_a_dead_key_where_the_sheet_cannot_appear](/crates/oxide-app/src/app/dispatch/input/f1_is_a_dead_key_where_the_sheet_cannot_appear.md) |
| related | [the_recorder_claims_only_where_preferences_is_painted_inline](/crates/oxide-app/src/app/dispatch/input/the_recorder_claims_only_where_preferences_is_painted_inline.md) |
| related | [a_detached_preferences_moves_the_recorder_with_it](/crates/oxide-app/src/app/dispatch/input/a_detached_preferences_moves_the_recorder_with_it.md) |
| related | [selection_slot_only_matches_digits_one_through_eight](/crates/oxide-app/src/app/dispatch/input/selection_slot_only_matches_digits_one_through_eight.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
