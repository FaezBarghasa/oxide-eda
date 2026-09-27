---
okf_version: "0.2"
type: Module
title: symbol
description: Symbol-editor + SCH-library-panel state mutators — the helper
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol
language: rust
---

# symbol

Symbol-editor + SCH-library-panel state mutators — the helper

## Docstring

Symbol-editor + SCH-library-panel state mutators — the helper
methods behind the `SchLibrary*` and `SymEditor*` dock-panel
messages. Each resolves the active `.snxsym` tab, mutates its
`SymbolEditorState`, marks the tab dirty, and clears the canvas
cache; the dispatcher in `mod.rs` routes the panel messages here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [sym_editor_mutate_display](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_display.md) |
| related | [sym_editor_mutate_pin](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_pin.md) |
| related | [sym_editor_mutate_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_symbol.md) |
| related | [sym_editor_mutate_graphic](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_graphic.md) |
| related | [sym_editor_toggle_graphic_fill_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_toggle_graphic_fill_picker.md) |
| related | [sym_editor_open_graphic_fill_advanced](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_open_graphic_fill_advanced.md) |
| related | [sym_editor_cancel_graphic_fill_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_cancel_graphic_fill_picker.md) |
| related | [sym_editor_set_graphic_fill](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_graphic_fill.md) |
| related | [sym_editor_toggle_local_color_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_toggle_local_color_picker.md) |
| related | [sym_editor_open_local_color_advanced](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_open_local_color_advanced.md) |
| related | [sym_editor_cancel_local_color_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_cancel_local_color_picker.md) |
| related | [sym_editor_set_local_color](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_local_color.md) |
| related | [sym_editor_select_graphic](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_graphic.md) |
| related | [sym_editor_select_part](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_part.md) |
| related | [sym_editor_select_pin](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_pin.md) |
| related | [sym_editor_set_pin_electrical](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_electrical.md) |
| related | [sym_editor_set_pin_orientation](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_orientation.md) |
| related | [sym_editor_set_pin_x](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_x.md) |
| related | [sym_editor_set_pin_y](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_y.md) |
| related | [sym_editor_set_pin_number](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_number.md) |
| related | [sym_editor_set_pin_name](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_name.md) |
| related | [sym_editor_set_pin_length](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_length.md) |
| related | [sym_editor_set_symbol_name](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_symbol_name.md) |
| related | [mark_active_symbol_tab_dirty](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/mark_active_symbol_tab_dirty.md) |
| related | [reset_symbol_viewport](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/reset_symbol_viewport.md) |
| related | [sch_library_select_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_select_symbol.md) |
| related | [sch_library_add_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_add_symbol.md) |
| related | [sch_library_delete_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_delete_symbol.md) |
| related | [active_symbol_editor_mut](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/active_symbol_editor_mut.md) |
| related | [sym_editor_mutate_display](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_display.md) |
| related | [sym_editor_mutate_pin](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_pin.md) |
| related | [sym_editor_mutate_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_symbol.md) |
| related | [sym_editor_mutate_graphic](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_graphic.md) |
| related | [sym_editor_toggle_graphic_fill_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_toggle_graphic_fill_picker.md) |
| related | [sym_editor_open_graphic_fill_advanced](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_open_graphic_fill_advanced.md) |
| related | [sym_editor_cancel_graphic_fill_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_cancel_graphic_fill_picker.md) |
| related | [sym_editor_set_graphic_fill](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_graphic_fill.md) |
| related | [sym_editor_toggle_local_color_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_toggle_local_color_picker.md) |
| related | [sym_editor_open_local_color_advanced](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_open_local_color_advanced.md) |
| related | [sym_editor_cancel_local_color_picker](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_cancel_local_color_picker.md) |
| related | [sym_editor_set_local_color](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_local_color.md) |
| related | [sym_editor_select_graphic](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_graphic.md) |
| related | [sym_editor_select_part](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_part.md) |
| related | [sym_editor_select_pin](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_pin.md) |
| related | [sym_editor_set_pin_electrical](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_electrical.md) |
| related | [sym_editor_set_pin_orientation](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_orientation.md) |
| related | [sym_editor_set_pin_x](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_x.md) |
| related | [sym_editor_set_pin_y](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_y.md) |
| related | [sym_editor_set_pin_number](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_number.md) |
| related | [sym_editor_set_pin_name](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_name.md) |
| related | [sym_editor_set_pin_length](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_pin_length.md) |
| related | [sym_editor_set_symbol_name](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_symbol_name.md) |
| related | [mark_active_symbol_tab_dirty](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/mark_active_symbol_tab_dirty.md) |
| related | [reset_symbol_viewport](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/reset_symbol_viewport.md) |
| related | [sch_library_select_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_select_symbol.md) |
| related | [sch_library_add_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_add_symbol.md) |
| related | [sch_library_delete_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_delete_symbol.md) |
| related | [active_symbol_editor_mut](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/active_symbol_editor_mut.md) |
