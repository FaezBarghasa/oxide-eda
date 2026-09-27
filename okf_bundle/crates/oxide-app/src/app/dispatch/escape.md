---
okf_version: "0.2"
type: Module
title: escape
description: "Esc resolution: which OS window the key was typed in, and what Esc"
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape
language: rust
---

# escape

Esc resolution: which OS window the key was typed in, and what Esc

## Docstring

Esc resolution: which OS window the key was typed in, and what Esc
means there.

Split out of the `Message::EscapePressed` arm in `dispatch/mod.rs`
(#554), which had grown into ~110 inline lines that read the MAIN
window's state no matter where the key came from.

# Windows are not interchangeable

[`InputTarget`] is derived from `WindowKind` by an exhaustive match,
so a new window kind is a compile error here rather than silently
inheriting the main window's Esc — which is how #547 and this issue
both happened.

The four kinds split by **what they paint**, not by what they are:

* an undocked tab renders a full duplicate of the main view, overlay
stack included (`view/mod.rs` maps `UndockedTab { .. } =>
view_main_for(window_id)`, and `view_main_for` pushes
`collect_overlays()` unconditionally), so the Esc ladder is
*correct* there — a modal it dismisses really is on screen in that
window;
* a detached modal paints one modal and no stack (#547);
* a detached panel and a component-editor window paint neither, so
the ladder must not run for them at all.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_escape_pressed](/crates/oxide-app/src/app/dispatch/escape/handle_escape_pressed.md) |
| related | [escape_editor_or_tool_reset](/crates/oxide-app/src/app/dispatch/escape/escape_editor_or_tool_reset.md) |
| related | [escape_tool_reset](/crates/oxide-app/src/app/dispatch/escape/escape_tool_reset.md) |
| related | [handle_escape_pressed](/crates/oxide-app/src/app/dispatch/escape/handle_escape_pressed.md) |
| related | [escape_editor_or_tool_reset](/crates/oxide-app/src/app/dispatch/escape/escape_editor_or_tool_reset.md) |
| related | [escape_tool_reset](/crates/oxide-app/src/app/dispatch/escape/escape_tool_reset.md) |
| related | [quiet_app](/crates/oxide-app/src/app/dispatch/escape/quiet_app.md) |
| related | [open_window](/crates/oxide-app/src/app/dispatch/escape/open_window.md) |
| related | [detach](/crates/oxide-app/src/app/dispatch/escape/detach.md) |
| related | [esc_in_a_detached_modal_window_addresses_that_modal_and_not_the_canvas](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_detached_modal_window_addresses_that_modal_and_not_the_canvas.md) |
| related | [esc_in_the_main_window_still_leaves_a_detached_modal_alone](/crates/oxide-app/src/app/dispatch/escape/esc_in_the_main_window_still_leaves_a_detached_modal_alone.md) |
| related | [a_synthesised_esc_resolves_as_a_main_window_one](/crates/oxide-app/src/app/dispatch/escape/a_synthesised_esc_resolves_as_a_main_window_one.md) |
| related | [a_detached_modal_with_no_rung_still_swallows_its_own_esc](/crates/oxide-app/src/app/dispatch/escape/a_detached_modal_with_no_rung_still_swallows_its_own_esc.md) |
| related | [escape_source_classifies_every_window_kind](/crates/oxide-app/src/app/dispatch/escape/escape_source_classifies_every_window_kind.md) |
| related | [esc_in_an_undocked_tab_window_still_runs_the_ladder](/crates/oxide-app/src/app/dispatch/escape/esc_in_an_undocked_tab_window_still_runs_the_ladder.md) |
| related | [an_undocked_tabs_esc_never_reaches_the_main_windows_footprint_editor](/crates/oxide-app/src/app/dispatch/escape/an_undocked_tabs_esc_never_reaches_the_main_windows_footprint_editor.md) |
| related | [esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset](/crates/oxide-app/src/app/dispatch/escape/esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset.md) |
| related | [esc_in_a_detached_panel_window_changes_nothing](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_detached_panel_window_changes_nothing.md) |
| related | [esc_in_a_component_editor_window_changes_nothing](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_component_editor_window_changes_nothing.md) |
| related | [cancelling_a_placement_session_clears_the_ghost_in_every_window](/crates/oxide-app/src/app/dispatch/escape/cancelling_a_placement_session_clears_the_ghost_in_every_window.md) |
