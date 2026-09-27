---
okf_version: "0.2"
type: Module
title: tests
description: "Data-to-view tests (iced-rust skill §10): assert on the pure context-"
resource: crates/oxide-app/src/app/view/context_menu/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/view/context_menu/tests
language: rust
---

# tests

Data-to-view tests (iced-rust skill §10): assert on the pure context-

## Docstring

Data-to-view tests (iced-rust skill §10): assert on the pure context-
menu entry builders — no GPU, no window, no widget tree. Each builder
turns app state into a `Vec<DropdownEntry<Message>>`; the widget that
renders those rows is tested in `oxide-widgets`.

## Relationships

| Type | Target |
|------|--------|
| related | [labels](/crates/oxide-app/src/app/view/context_menu/tests/labels.md) |
| related | [has_row](/crates/oxide-app/src/app/view/context_menu/tests/has_row.md) |
| related | [seps](/crates/oxide-app/src/app/view/context_menu/tests/seps.md) |
| related | [customs](/crates/oxide-app/src/app/view/context_menu/tests/customs.md) |
| related | [disabled_of](/crates/oxide-app/src/app/view/context_menu/tests/disabled_of.md) |
| related | [shortcuts](/crates/oxide-app/src/app/view/context_menu/tests/shortcuts.md) |
| related | [dd_disabled_row_is_passive](/crates/oxide-app/src/app/view/context_menu/tests/dd_disabled_row_is_passive.md) |
| related | [dd_msg_row_carries_message_and_optional_shortcut](/crates/oxide-app/src/app/view/context_menu/tests/dd_msg_row_carries_message_and_optional_shortcut.md) |
| related | [canvas_menu_grows_and_gates_on_selection](/crates/oxide-app/src/app/view/context_menu/tests/canvas_menu_grows_and_gates_on_selection.md) |
| related | [canvas_child_sheet_adds_open_row](/crates/oxide-app/src/app/view/context_menu/tests/canvas_child_sheet_adds_open_row.md) |
| related | [tab_menu_gates_bulk_close_and_undock](/crates/oxide-app/src/app/view/context_menu/tests/tab_menu_gates_bulk_close_and_undock.md) |
| related | [tree_role_precedence](/crates/oxide-app/src/app/view/context_menu/tests/tree_role_precedence.md) |
| related | [align_gate_thresholds](/crates/oxide-app/src/app/view/context_menu/tests/align_gate_thresholds.md) |
| related | [align_menu_disables_pairwise_below_two](/crates/oxide-app/src/app/view/context_menu/tests/align_menu_disables_pairwise_below_two.md) |
| related | [place_menu_is_all_enabled_active_bar_rows](/crates/oxide-app/src/app/view/context_menu/tests/place_menu_is_all_enabled_active_bar_rows.md) |
