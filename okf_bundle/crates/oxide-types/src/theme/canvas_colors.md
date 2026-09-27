---
okf_version: "0.2"
type: Function
title: canvas_colors
resource: crates/oxide-types/src/theme.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/theme/canvas_colors
language: rust
---

# canvas_colors

## Signature

```rust
pub fn canvas_colors(id: ThemeId) -> CanvasColors
```

## Visibility

- `pub`

## Source
Lines 468–481 in `crates/oxide-types/src/theme.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme](/crates/oxide-types/src/theme.md) |
| called_by | [handle_print_preview_requested](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_requested.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
| called_by | [canvas_colors_for](/crates/oxide-app/src/app/runtime/mod/canvas_colors_for.md) |
| called_by | [update_canvas_theme](/crates/oxide-app/src/app/runtime/mod/update_canvas_theme.md) |
| called_by | [test_prefs](/crates/oxide-app/src/canvas/mod/test_prefs.md) |
| called_by | [draw_symbol_with_renderer](/crates/oxide-app/src/library/editor/symbol/canvas/mod/draw_symbol_with_renderer.md) |
| called_by | [new](/crates/oxide-app/src/pcb_canvas/new.md) |
| called_by | [draw_label_preview](/crates/oxide-app/src/schematic_runtime/label/draw_label_preview.md) |
| called_by | [draw_power_port_preview](/crates/oxide-app/src/schematic_runtime/mod/draw_power_port_preview.md) |
| called_by | [draw_erc_markers](/crates/oxide-app/src/schematic_runtime/overlay/draw_erc_markers.md) |
| called_by | [draw_selection_overlay](/crates/oxide-app/src/schematic_runtime/selection/draw_selection_overlay.md) |
| called_by | [draw_text_note_preview](/crates/oxide-app/src/schematic_runtime/text/draw_text_note_preview.md) |
| called_by | [custom_theme_json](/crates/oxide-app/tests/regression/preferences_dirty_guard/custom_theme_json.md) |
| called_by | [theme_import_dirty_flag_survives_an_unrelated_appearance_toggle](/crates/oxide-app/tests/regression/preferences_dirty_guard/theme_import_dirty_flag_survives_an_unrelated_appearance_toggle.md) |
| called_by | [altium_dark_paper_is_dark](/crates/oxide-output/src/pdf/palette/altium_dark_paper_is_dark.md) |
| called_by | [from_canvas_colors_normalises_u8_rgb](/crates/oxide-output/src/pdf/palette/from_canvas_colors_normalises_u8_rgb.md) |
| called_by | [from_theme_id](/crates/oxide-renderer/src/theme/from_theme_id.md) |
