---
okf_version: "0.2"
type: Function
title: quiet_app
description: A freshly built app with nothing claiming Esc.
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape/quiet_app
language: rust
---

# quiet_app

A freshly built app with nothing claiming Esc.

## Signature

```rust
fn quiet_app() -> Oxide
```

## Docstring

A freshly built app with nothing claiming Esc.

`Oxide::new()` opens the first-run tour, which is a legitimate
rung and would answer every Esc these tests send. Closing it is
the whole fixture; the assertion keeps that honest if another
overlay ever starts life open.

## Source
Lines 205–214 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
| called_by | [a_detached_modal_with_no_rung_still_swallows_its_own_esc](/crates/oxide-app/src/app/dispatch/escape/a_detached_modal_with_no_rung_still_swallows_its_own_esc.md) |
| called_by | [a_synthesised_esc_resolves_as_a_main_window_one](/crates/oxide-app/src/app/dispatch/escape/a_synthesised_esc_resolves_as_a_main_window_one.md) |
| called_by | [an_undocked_tabs_esc_never_reaches_the_main_windows_footprint_editor](/crates/oxide-app/src/app/dispatch/escape/an_undocked_tabs_esc_never_reaches_the_main_windows_footprint_editor.md) |
| called_by | [cancelling_a_placement_session_clears_the_ghost_in_every_window](/crates/oxide-app/src/app/dispatch/escape/cancelling_a_placement_session_clears_the_ghost_in_every_window.md) |
| called_by | [esc_in_a_component_editor_window_changes_nothing](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_component_editor_window_changes_nothing.md) |
| called_by | [esc_in_a_detached_modal_window_addresses_that_modal_and_not_the_canvas](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_detached_modal_window_addresses_that_modal_and_not_the_canvas.md) |
| called_by | [esc_in_a_detached_panel_window_changes_nothing](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_detached_panel_window_changes_nothing.md) |
| called_by | [esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset](/crates/oxide-app/src/app/dispatch/escape/esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset.md) |
| called_by | [esc_in_an_undocked_tab_window_still_runs_the_ladder](/crates/oxide-app/src/app/dispatch/escape/esc_in_an_undocked_tab_window_still_runs_the_ladder.md) |
| called_by | [esc_in_the_main_window_still_leaves_a_detached_modal_alone](/crates/oxide-app/src/app/dispatch/escape/esc_in_the_main_window_still_leaves_a_detached_modal_alone.md) |
| called_by | [escape_source_classifies_every_window_kind](/crates/oxide-app/src/app/dispatch/escape/escape_source_classifies_every_window_kind.md) |
