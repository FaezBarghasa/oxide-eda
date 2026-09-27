---
okf_version: "0.2"
type: Module
title: keymap
description: Keymap chord resolution.
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap
language: rust
---

# keymap

Keymap chord resolution.

## Docstring

Keymap chord resolution.

The keyboard subscription forwards each raw keystroke as
[`UiMsg::KeymapStroke`]. Resolution happens here, in `update`, where
`&mut self` is available — so the multi-stroke chord buffer lives in
[`UiState::keymap_pending_sequence`] instead of a process-global
static (sound across multiple windows, MVU-clean).

A resolved command id is run through [`Oxide::dispatch_command`] —
the Command Registry's dispatch entry point (#278) — so the
keyboard is a plain consumer of the registry rather than owning
bridge logic itself.

## Relationships

| Type | Target |
|------|--------|
| related | [resolve_keymap_stroke](/crates/oxide-app/src/app/dispatch/keymap/resolve_keymap_stroke.md) |
| related | [take_keymap_match](/crates/oxide-app/src/app/dispatch/keymap/take_keymap_match.md) |
| related | [begin_or_continue_chord](/crates/oxide-app/src/app/dispatch/keymap/begin_or_continue_chord.md) |
| related | [shortcut_contexts](/crates/oxide-app/src/app/dispatch/keymap/shortcut_contexts.md) |
| related | [resolve_keymap_stroke](/crates/oxide-app/src/app/dispatch/keymap/resolve_keymap_stroke.md) |
| related | [take_keymap_match](/crates/oxide-app/src/app/dispatch/keymap/take_keymap_match.md) |
| related | [begin_or_continue_chord](/crates/oxide-app/src/app/dispatch/keymap/begin_or_continue_chord.md) |
| related | [shortcut_contexts](/crates/oxide-app/src/app/dispatch/keymap/shortcut_contexts.md) |
| related | [app_with_footprint_tab](/crates/oxide-app/src/app/dispatch/keymap/app_with_footprint_tab.md) |
| related | [open_window](/crates/oxide-app/src/app/dispatch/keymap/open_window.md) |
| related | [undocked](/crates/oxide-app/src/app/dispatch/keymap/undocked.md) |
| related | [an_undocked_window_does_not_inherit_the_main_windows_editor_contexts](/crates/oxide-app/src/app/dispatch/keymap/an_undocked_window_does_not_inherit_the_main_windows_editor_contexts.md) |
| related | [windows_that_paint_no_document_get_global_only](/crates/oxide-app/src/app/dispatch/keymap/windows_that_paint_no_document_get_global_only.md) |
| related | [a_synthesised_stroke_resolves_as_the_main_window](/crates/oxide-app/src/app/dispatch/keymap/a_synthesised_stroke_resolves_as_the_main_window.md) |
| related | [a_chord_prefix_does_not_survive_a_change_of_window](/crates/oxide-app/src/app/dispatch/keymap/a_chord_prefix_does_not_survive_a_change_of_window.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
