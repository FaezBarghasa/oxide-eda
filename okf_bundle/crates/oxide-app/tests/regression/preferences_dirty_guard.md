---
okf_version: "0.2"
type: Module
title: preferences_dirty_guard
description: "Review #308 findings 1 + 2 — Preferences dirty-tracking must not miss an"
resource: crates/oxide-app/tests/regression/preferences_dirty_guard.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_dirty_guard
language: rust
---

# preferences_dirty_guard

Review #308 findings 1 + 2 — Preferences dirty-tracking must not miss an

## Docstring

Review #308 findings 1 + 2 — Preferences dirty-tracking must not miss an
imperative edit, and no dismiss route may discard unsaved changes without
the user being asked (or at least without being allowed to silently
proceed). See `crate::app::state::UiState::preferences_draft_differs` /
`preferences_has_unsaved_changes` and
`handle_preferences_close_requested` / `WindowMsg::WindowCloseRequested`.

## Relationships

| Type | Target |
|------|--------|
| related | [custom_theme_json](/crates/oxide-app/tests/regression/preferences_dirty_guard/custom_theme_json.md) |
| related | [theme_import_dirty_flag_survives_an_unrelated_appearance_toggle](/crates/oxide-app/tests/regression/preferences_dirty_guard/theme_import_dirty_flag_survives_an_unrelated_appearance_toggle.md) |
| related | [keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle](/crates/oxide-app/tests/regression/preferences_dirty_guard/keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle.md) |
| related | [os_close_request_is_refused_while_preferences_is_dirty](/crates/oxide-app/tests/regression/preferences_dirty_guard/os_close_request_is_refused_while_preferences_is_dirty.md) |
| related | [os_close_request_proceeds_when_preferences_is_clean](/crates/oxide-app/tests/regression/preferences_dirty_guard/os_close_request_proceeds_when_preferences_is_clean.md) |
