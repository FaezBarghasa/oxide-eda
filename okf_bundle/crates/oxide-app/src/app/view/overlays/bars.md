---
okf_version: "0.2"
type: Module
title: bars
description: "Editor-surface overlay builders — the blocking-modal gate, the"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars
language: rust
---

# bars

Editor-surface overlay builders — the blocking-modal gate, the

## Docstring

Editor-surface overlay builders — the blocking-modal gate, the
export/preview/net-colour top-of-stack overlays, the schematic /
footprint / symbol active bars, the in-canvas text-edit input, and
the status-bar panel list.

Extracted from the former 1223-line `collect_overlays` god-function
in `view/mod.rs` (behaviour-preserving decomposition). Each builder
owns one overlay's guard + widget tree; `collect_overlays` is now a
thin assembler that calls them in push order. These are methods of
the same `Oxide` view impl, split across sibling files.

## Relationships

| Type | Target |
|------|--------|
| related | [has_blocking_modal](/crates/oxide-app/src/app/view/overlays/bars/has_blocking_modal.md) |
| related | [error_notice_overlay](/crates/oxide-app/src/app/view/overlays/bars/error_notice_overlay.md) |
| related | [netlist_incomplete_prompt_overlay](/crates/oxide-app/src/app/view/overlays/bars/netlist_incomplete_prompt_overlay.md) |
| related | [print_preview_overlay](/crates/oxide-app/src/app/view/overlays/bars/print_preview_overlay.md) |
| related | [bom_preview_overlay](/crates/oxide-app/src/app/view/overlays/bars/bom_preview_overlay.md) |
| related | [net_color_custom_overlay](/crates/oxide-app/src/app/view/overlays/bars/net_color_custom_overlay.md) |
| related | [placement_paused_overlay](/crates/oxide-app/src/app/view/overlays/bars/placement_paused_overlay.md) |
| related | [schematic_active_bar_overlay](/crates/oxide-app/src/app/view/overlays/bars/schematic_active_bar_overlay.md) |
| related | [footprint_active_bar_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_active_bar_overlay.md) |
| related | [footprint_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_context_menu_overlay.md) |
| related | [footprint_move_by_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_move_by_overlay.md) |
| related | [footprint_align_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_align_overlay.md) |
| related | [symbol_editor_active_bar_overlay](/crates/oxide-app/src/app/view/overlays/bars/symbol_editor_active_bar_overlay.md) |
| related | [symbol_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/bars/symbol_context_menu_overlay.md) |
| related | [clamp_symbol_menu_position](/crates/oxide-app/src/app/view/overlays/bars/clamp_symbol_menu_position.md) |
| related | [text_edit_overlay](/crates/oxide-app/src/app/view/overlays/bars/text_edit_overlay.md) |
| related | [panel_list_overlay](/crates/oxide-app/src/app/view/overlays/bars/panel_list_overlay.md) |
| related | [has_blocking_modal](/crates/oxide-app/src/app/view/overlays/bars/has_blocking_modal.md) |
| related | [error_notice_overlay](/crates/oxide-app/src/app/view/overlays/bars/error_notice_overlay.md) |
| related | [netlist_incomplete_prompt_overlay](/crates/oxide-app/src/app/view/overlays/bars/netlist_incomplete_prompt_overlay.md) |
| related | [print_preview_overlay](/crates/oxide-app/src/app/view/overlays/bars/print_preview_overlay.md) |
| related | [bom_preview_overlay](/crates/oxide-app/src/app/view/overlays/bars/bom_preview_overlay.md) |
| related | [net_color_custom_overlay](/crates/oxide-app/src/app/view/overlays/bars/net_color_custom_overlay.md) |
| related | [placement_paused_overlay](/crates/oxide-app/src/app/view/overlays/bars/placement_paused_overlay.md) |
| related | [schematic_active_bar_overlay](/crates/oxide-app/src/app/view/overlays/bars/schematic_active_bar_overlay.md) |
| related | [footprint_active_bar_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_active_bar_overlay.md) |
| related | [footprint_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_context_menu_overlay.md) |
| related | [footprint_move_by_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_move_by_overlay.md) |
| related | [footprint_align_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_align_overlay.md) |
| related | [symbol_editor_active_bar_overlay](/crates/oxide-app/src/app/view/overlays/bars/symbol_editor_active_bar_overlay.md) |
| related | [symbol_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/bars/symbol_context_menu_overlay.md) |
| related | [clamp_symbol_menu_position](/crates/oxide-app/src/app/view/overlays/bars/clamp_symbol_menu_position.md) |
| related | [text_edit_overlay](/crates/oxide-app/src/app/view/overlays/bars/text_edit_overlay.md) |
| related | [panel_list_overlay](/crates/oxide-app/src/app/view/overlays/bars/panel_list_overlay.md) |
| related | [blank_preview](/crates/oxide-app/src/app/view/overlays/bars/blank_preview.md) |
| related | [a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack](/crates/oxide-app/src/app/view/overlays/bars/a_detached_print_preview_stops_blocking_the_main_windows_overlay_stack.md) |
| related | [detaching_the_net_colour_palette_does_not_unblock_the_custom_picker](/crates/oxide-app/src/app/view/overlays/bars/detaching_the_net_colour_palette_does_not_unblock_the_custom_picker.md) |
| related | [the_painter_and_the_esc_ladder_agree_about_a_detached_preview](/crates/oxide-app/src/app/view/overlays/bars/the_painter_and_the_esc_ladder_agree_about_a_detached_preview.md) |
