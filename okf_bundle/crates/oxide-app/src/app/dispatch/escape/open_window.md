---
okf_version: "0.2"
type: Function
title: open_window
description: "Give `kind` its own OS window and hand back that window's id."
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape/open_window
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
Lines 217–221 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
| called_by | [an_undocked_tabs_esc_never_reaches_the_main_windows_footprint_editor](/crates/oxide-app/src/app/dispatch/escape/an_undocked_tabs_esc_never_reaches_the_main_windows_footprint_editor.md) |
| called_by | [detach](/crates/oxide-app/src/app/dispatch/escape/detach.md) |
| called_by | [esc_in_a_component_editor_window_changes_nothing](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_component_editor_window_changes_nothing.md) |
| called_by | [esc_in_a_detached_panel_window_changes_nothing](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_detached_panel_window_changes_nothing.md) |
| called_by | [esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset](/crates/oxide-app/src/app/dispatch/escape/esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset.md) |
| called_by | [esc_in_an_undocked_tab_window_still_runs_the_ladder](/crates/oxide-app/src/app/dispatch/escape/esc_in_an_undocked_tab_window_still_runs_the_ladder.md) |
| called_by | [escape_source_classifies_every_window_kind](/crates/oxide-app/src/app/dispatch/escape/escape_source_classifies_every_window_kind.md) |
