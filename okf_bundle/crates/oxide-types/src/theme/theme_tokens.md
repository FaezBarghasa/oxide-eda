---
okf_version: "0.2"
type: Function
title: theme_tokens
description: "---------------------------------------------------------------------------"
resource: crates/oxide-types/src/theme.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/theme/theme_tokens
language: rust
---

# theme_tokens

---------------------------------------------------------------------------

## Signature

```rust
pub fn theme_tokens(id: ThemeId) -> ThemeTokens
```

## Visibility

- `pub`

## Docstring

---------------------------------------------------------------------------
Public accessors
---------------------------------------------------------------------------

## Source
Lines 455–466 in `crates/oxide-types/src/theme.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme](/crates/oxide-types/src/theme.md) |
| called_by | [new](/crates/chrome-catalog/src/catalog/new.md) |
| called_by | [update](/crates/chrome-catalog/src/catalog/update.md) |
| called_by | [dropdown_entries_match_pre_refactor_golden](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries_match_pre_refactor_golden.md) |
| called_by | [filter_menu_is_a_single_custom_entry](/crates/oxide-app/src/active_bar/dropdown/filter_menu_is_a_single_custom_entry.md) |
| called_by | [net_color_swatches_and_gated_clear_rows](/crates/oxide-app/src/active_bar/dropdown/net_color_swatches_and_gated_clear_rows.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
| called_by | [revert_preferences_drafts](/crates/oxide-app/src/app/handlers/preferences/mod/revert_preferences_drafts.md) |
| called_by | [refresh_panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx.md) |
| called_by | [canvas_child_sheet_adds_open_row](/crates/oxide-app/src/app/view/context_menu/tests/canvas_child_sheet_adds_open_row.md) |
| called_by | [canvas_menu_grows_and_gates_on_selection](/crates/oxide-app/src/app/view/context_menu/tests/canvas_menu_grows_and_gates_on_selection.md) |
| called_by | [accent_hex](/crates/oxide-app/src/icons/accent_hex.md) |
| called_by | [place_move_button](/crates/oxide-app/src/library/editor/footprint/tests/place_move_button.md) |
| called_by | [bar_width_counts_every_slot_including_the_custom_one](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_width_counts_every_slot_including_the_custom_one.md) |
| called_by | [menu_triggers_are_located_by_message_not_by_index](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_triggers_are_located_by_message_not_by_index.md) |
| called_by | [custom_theme_json](/crates/oxide-app/tests/regression/preferences_dirty_guard/custom_theme_json.md) |
| called_by | [theme_import_dirty_flag_survives_an_unrelated_appearance_toggle](/crates/oxide-app/tests/regression/preferences_dirty_guard/theme_import_dirty_flag_survives_an_unrelated_appearance_toggle.md) |
