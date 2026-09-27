---
okf_version: "0.2"
type: Function
title: write_ui_font_pref
description: "Persist the given `ui_font` choice to the preferences file."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/write_ui_font_pref
language: rust
---

# write_ui_font_pref

Persist the given `ui_font` choice to the preferences file.

## Signature

```rust
pub fn write_ui_font_pref(font_name: &str)
```

## Visibility

- `pub`

## Docstring

Persist the given `ui_font` choice to the preferences file.
Creates parent directories if they do not exist.
Silently ignores I/O errors (non-critical preference).

## Source
Lines 392–394 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [write_ui_font_pref_at](/crates/oxide-app/src/fonts/mod/write_ui_font_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [handle_dock_panel_control_message](/crates/oxide-app/src/app/handlers/dock/panel_controls/handle_dock_panel_control_message.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
