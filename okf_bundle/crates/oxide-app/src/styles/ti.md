---
okf_version: "0.2"
type: Function
title: ti
description: Convert a oxide-types Color to an iced Color.
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/ti
language: rust
---

# ti

Convert a oxide-types Color to an iced Color.

## Signature

```rust
pub fn ti(c: oxide_types::theme::Color) -> Color
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Convert a oxide-types Color to an iced Color.
[inline]

## Source
Lines 15–17 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| called_by | [filter_entry](/crates/oxide-app/src/active_bar/dropdown/filter_entry.md) |
| called_by | [from_tokens](/crates/oxide-app/src/active_bar/mod/from_tokens.md) |
| called_by | [view_main_window_chrome](/crates/oxide-app/src/app/view/chrome/view_main_window_chrome.md) |
| called_by | [submenu_launcher](/crates/oxide-app/src/app/view/context_menu/items/submenu_launcher.md) |
| called_by | [view_annotate_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_dialog_body_inner.md) |
| called_by | [view_annotate_reset_confirm_body_inner](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_reset_confirm_body_inner.md) |
| called_by | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| called_by | [bom_sidebar](/crates/oxide-app/src/app/view/dialogs/bom/sidebar/bom_sidebar.md) |
| called_by | [bom_table](/crates/oxide-app/src/app/view/dialogs/bom/table/bom_table.md) |
| called_by | [view_app_quit_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm_body.md) |
| called_by | [view_project_close_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_project_close_confirm_body.md) |
| called_by | [view_remove_dialog_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_remove_dialog_body.md) |
| called_by | [pin_matrix_view](/crates/oxide-app/src/app/view/dialogs/erc/pin_matrix_view.md) |
| called_by | [view_erc_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog_body_inner.md) |
| called_by | [view_enable_version_control_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_enable_version_control_dialog_body.md) |
| called_by | [view_grid_properties_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_grid_properties_dialog_body.md) |
| called_by | [view_project_options_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_project_options_dialog_body.md) |
| called_by | [view_rename_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_rename_dialog_body.md) |
| called_by | [view_selection_filter_custom_body](/crates/oxide-app/src/app/view/dialogs/project/view_selection_filter_custom_body.md) |
| called_by | [view_center](/crates/oxide-app/src/app/view/mod/view_center.md) |
| called_by | [view_tab_drag_ghost](/crates/oxide-app/src/app/view/mod/view_tab_drag_ghost.md) |
| called_by | [view_close_x](/crates/oxide-app/src/app/view/modals/view_close_x.md) |
| called_by | [view_move_selection_body](/crates/oxide-app/src/app/view/modals/view_move_selection_body.md) |
| called_by | [view_net_color_custom_picker](/crates/oxide-app/src/app/view/modals/view_net_color_custom_picker.md) |
| called_by | [view_net_color_palette_body](/crates/oxide-app/src/app/view/modals/view_net_color_palette_body.md) |
| called_by | [view_parameter_manager_body](/crates/oxide-app/src/app/view/modals/view_parameter_manager_body.md) |
| called_by | [panel_list_overlay](/crates/oxide-app/src/app/view/overlays/bars/panel_list_overlay.md) |
| called_by | [placement_paused_overlay](/crates/oxide-app/src/app/view/overlays/bars/placement_paused_overlay.md) |
| called_by | [text_edit_overlay](/crates/oxide-app/src/app/view/overlays/bars/text_edit_overlay.md) |
| called_by | [view_command_palette_dropdown](/crates/oxide-app/src/app/view/overlays/mod/view_command_palette_dropdown.md) |
| called_by | [view_hover_tooltip](/crates/oxide-app/src/app/view/overlays/mod/view_hover_tooltip.md) |
| called_by | [view_pdf_tab_strip](/crates/oxide-app/src/app/view/pdf_preview/mod/view_pdf_tab_strip.md) |
| called_by | [view_pdf_preview_tab](/crates/oxide-app/src/app/view/pdf_preview/preview/view_pdf_preview_tab.md) |
| called_by | [pdf_section_title](/crates/oxide-app/src/app/view/pdf_preview/settings/pdf_section_title.md) |
| called_by | [view_pdf_additional_section](/crates/oxide-app/src/app/view/pdf_preview/settings/view_pdf_additional_section.md) |
| called_by | [view_pdf_files_section](/crates/oxide-app/src/app/view/pdf_preview/settings/view_pdf_files_section.md) |
| called_by | [view_pdf_structure_section](/crates/oxide-app/src/app/view/pdf_preview/settings/view_pdf_structure_section.md) |
| called_by | [view_error_notice](/crates/oxide-app/src/app/view/print_preview/view_error_notice.md) |
| called_by | [view_netlist_incomplete_prompt](/crates/oxide-app/src/app/view/print_preview/view_netlist_incomplete_prompt.md) |
| called_by | [view_print_preview_inner](/crates/oxide-app/src/app/view/print_preview/view_print_preview_inner.md) |
| called_by | [view_rail](/crates/oxide-app/src/dock/view/view_rail.md) |
| called_by | [view_region](/crates/oxide-app/src/dock/view/view_region.md) |
| called_by | [view](/crates/oxide-app/src/find_replace/view.md) |
| called_by | [view](/crates/oxide-app/src/first_run_tour/view.md) |
| called_by | [view](/crates/oxide-app/src/keyboard_shortcuts_modal/view.md) |
| called_by | [view_align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal/view_align_modal.md) |
| called_by | [item_disabled](/crates/oxide-app/src/library/editor/footprint/context_menu/item_disabled.md) |
| called_by | [item_indented](/crates/oxide-app/src/library/editor/footprint/context_menu/item_indented.md) |
| called_by | [item_msg](/crates/oxide-app/src/library/editor/footprint/context_menu/item_msg.md) |
| called_by | [item_submenu_header](/crates/oxide-app/src/library/editor/footprint/context_menu/item_submenu_header.md) |
| called_by | [separator](/crates/oxide-app/src/library/editor/footprint/context_menu/separator.md) |
| called_by | [view_context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu/view_context_menu.md) |
| called_by | [view_move_by_modal](/crates/oxide-app/src/library/editor/footprint/move_by_modal/view_move_by_modal.md) |
| called_by | [view_symbol_canvas](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_canvas.md) |
| called_by | [view](/crates/oxide-app/src/library/picker/view.md) |
| called_by | [view](/crates/oxide-app/src/library/settings/distributor_apis/view.md) |
| called_by | [from_tokens](/crates/oxide-app/src/menu_bar/mod/from_tokens.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| called_by | [view_library_block](/crates/oxide-app/src/panels/components_panel/mod/view_library_block.md) |
| called_by | [view_section](/crates/oxide-app/src/panels/components_panel/mod/view_section.md) |
| called_by | [view_footprint_library](/crates/oxide-app/src/panels/library/view_footprint_library.md) |
| called_by | [view_sch_library](/crates/oxide-app/src/panels/library/view_sch_library.md) |
| called_by | [from_tokens](/crates/oxide-app/src/panels/palette/from_tokens.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
| called_by | [view](/crates/oxide-app/src/passive_calculator_modal/view.md) |
| called_by | [view](/crates/oxide-app/src/status_bar/view.md) |
| called_by | [active_bar_strip](/crates/oxide-app/src/styles/active_bar_strip.md) |
| called_by | [chrome_separator](/crates/oxide-app/src/styles/chrome_separator.md) |
| called_by | [collapsed_rail](/crates/oxide-app/src/styles/collapsed_rail.md) |
| called_by | [context_menu](/crates/oxide-app/src/styles/context_menu.md) |
| called_by | [dock_tab_container](/crates/oxide-app/src/styles/dock_tab_container.md) |
| called_by | [dock_zone_highlight](/crates/oxide-app/src/styles/dock_zone_highlight.md) |
| called_by | [floating_panel_body](/crates/oxide-app/src/styles/floating_panel_body.md) |
| called_by | [floating_title_bar](/crates/oxide-app/src/styles/floating_title_bar.md) |
| called_by | [menu_item](/crates/oxide-app/src/styles/menu_item.md) |
| called_by | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [modal_footer_strip](/crates/oxide-app/src/styles/modal_footer_strip.md) |
| called_by | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| called_by | [panel_card](/crates/oxide-app/src/styles/panel_card.md) |
| called_by | [panel_content](/crates/oxide-app/src/styles/panel_content.md) |
| called_by | [panel_region](/crates/oxide-app/src/styles/panel_region.md) |
| called_by | [rail_tab](/crates/oxide-app/src/styles/rail_tab.md) |
| called_by | [resize_handle](/crates/oxide-app/src/styles/resize_handle.md) |
| called_by | [status_bar](/crates/oxide-app/src/styles/status_bar.md) |
| called_by | [tab_bar_strip](/crates/oxide-app/src/styles/tab_bar_strip.md) |
| called_by | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |
| called_by | [pill_fill](/crates/oxide-app/src/tab_bar/pill_fill.md) |
| called_by | [view](/crates/oxide-app/src/tab_bar/view.md) |
