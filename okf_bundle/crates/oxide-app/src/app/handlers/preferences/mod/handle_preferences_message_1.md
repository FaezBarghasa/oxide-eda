---
okf_version: "0.2"
type: Function
title: handle_preferences_message
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message_1
language: rust
---

# handle_preferences_message

## Signature

```rust
pub(crate) fn handle_preferences_message(
        &mut self,
        msg: crate::preferences::PrefMsg,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 182–752 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [write_ui_font_pref](/crates/oxide-app/src/fonts/mod/write_ui_font_pref.md) |
| calls | [write_power_port_style_pref](/crates/oxide-app/src/fonts/mod/write_power_port_style_pref.md) |
| calls | [write_label_style_pref](/crates/oxide-app/src/fonts/mod/write_label_style_pref.md) |
| calls | [write_multisheet_style_pref](/crates/oxide-app/src/fonts/mod/write_multisheet_style_pref.md) |
| calls | [write_grid_style_pref](/crates/oxide-app/src/fonts/mod/write_grid_style_pref.md) |
| calls | [write_pcb_gpu_render_pref](/crates/oxide-app/src/fonts/mod/write_pcb_gpu_render_pref.md) |
| calls | [write_theme_pref](/crates/oxide-app/src/fonts/mod/write_theme_pref.md) |
| calls | [write_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_size_mm_pref.md) |
| calls | [write_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_style_pref.md) |
| calls | [write_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/write_symbol_pin_selection_pref.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [write_component_classes_pref](/crates/oxide-app/src/fonts/mod/write_component_classes_pref.md) |
| calls | [back_up_profile_file](/crates/oxide-app/src/keymap/profile/back_up_profile_file.md) |
| calls | [save_profile_set](/crates/oxide-app/src/keymap/profile/save_profile_set.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [write_erc_severity_overrides](/crates/oxide-app/src/fonts/erc/write_erc_severity_overrides.md) |
| calls | [check_prefs_file](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file.md) |
| calls | [move_prefs_file_aside](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside.md) |
| calls | [default_component_classes](/crates/oxide-app/src/fonts/mod/default_component_classes.md) |
