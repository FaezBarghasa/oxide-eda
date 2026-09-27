---
okf_version: "0.2"
type: Function
title: text_secondary
description: Secondary / muted text color.
resource: crates/oxide-widgets/src/theme_ext.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/theme_ext/text_secondary
language: rust
---

# text_secondary

Secondary / muted text color.

## Signature

```rust
pub fn text_secondary(tokens: &ThemeTokens) -> Color
```

## Visibility

- `pub`

## Docstring

Secondary / muted text color.

## Source
Lines 29–31 in `crates/oxide-widgets/src/theme_ext.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_ext](/crates/oxide-widgets/src/theme_ext.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| called_by | [view_grid_picker_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_grid_picker_menu.md) |
| called_by | [view_action_row](/crates/oxide-app/src/library/browser/action_row/view_action_row.md) |
| called_by | [view_empty_state](/crates/oxide-app/src/library/browser/empty_state/view_empty_state.md) |
| called_by | [view_grid](/crates/oxide-app/src/library/browser/grid/view_grid.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
| called_by | [preview_panel](/crates/oxide-app/src/library/browser/preview/preview_panel.md) |
| called_by | [preview_panel_with_pick](/crates/oxide-app/src/library/browser/preview/preview_panel_with_pick.md) |
| called_by | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
| called_by | [view_table_sidebar](/crates/oxide-app/src/library/browser/sidebar/view_table_sidebar.md) |
| called_by | [close_x](/crates/oxide-app/src/library/close_prompt/close_x.md) |
| called_by | [view](/crates/oxide-app/src/library/close_prompt/view.md) |
| called_by | [view](/crates/oxide-app/src/library/create_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/document_options/view.md) |
| called_by | [close_x](/crates/oxide-app/src/library/edit_row_modal/close_x.md) |
| called_by | [view](/crates/oxide-app/src/library/edit_row_modal/view.md) |
| called_by | [view_params_section](/crates/oxide-app/src/library/edit_row_modal/view_params_section.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/datasheet_picker/view.md) |
| called_by | [view_pinned_input](/crates/oxide-app/src/library/editor/datasheet_picker/view_pinned_input.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/body3d/view.md) |
| called_by | [footprint_tabs_overlay](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/footprint_tabs_overlay.md) |
| called_by | [build_dimension_input](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/build_dimension_input.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/step_attach/view.md) |
| called_by | [close_btn](/crates/oxide-app/src/library/editor/mod/close_btn.md) |
| called_by | [placeholder_card](/crates/oxide-app/src/library/editor/mod/placeholder_card.md) |
| called_by | [view_footer](/crates/oxide-app/src/library/editor/mod/view_footer.md) |
| called_by | [view_header](/crates/oxide-app/src/library/editor/mod/view_header.md) |
| called_by | [add_custom_row](/crates/oxide-app/src/library/editor/params/add_custom_row.md) |
| called_by | [custom_row](/crates/oxide-app/src/library/editor/params/custom_row.md) |
| called_by | [template_row](/crates/oxide-app/src/library/editor/params/template_row.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/params/view.md) |
| called_by | [header_row](/crates/oxide-app/src/library/editor/preview/header_row.md) |
| called_by | [open_btn](/crates/oxide-app/src/library/editor/preview/open_btn.md) |
| called_by | [override_action_btn](/crates/oxide-app/src/library/editor/preview/override_action_btn.md) |
| called_by | [override_editor_row](/crates/oxide-app/src/library/editor/preview/override_editor_row.md) |
| called_by | [pin_map_subsection](/crates/oxide-app/src/library/editor/preview/pin_map_subsection.md) |
| called_by | [render_pane](/crates/oxide-app/src/library/editor/preview/render_pane.md) |
| called_by | [where_used_footer](/crates/oxide-app/src/library/editor/preview/where_used_footer.md) |
| called_by | [pin_node_row](/crates/oxide-app/src/library/editor/sim/mod/pin_node_row.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/sim/mod/view.md) |
| called_by | [view_pin_node_table](/crates/oxide-app/src/library/editor/sim/mod/view_pin_node_table.md) |
| called_by | [view_footprint_footer](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_footer.md) |
| called_by | [view_footprint_layers_strip](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_layers_strip.md) |
| called_by | [view_footprint_sketch_toolbar](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_sketch_toolbar.md) |
| called_by | [view_footprint_toolbar](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_toolbar.md) |
| called_by | [view_footprint_top_strip](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_top_strip.md) |
| called_by | [view_symbol_status](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_status.md) |
| called_by | [alternates_section](/crates/oxide-app/src/library/editor/supply/alternates_section.md) |
| called_by | [labelled_input](/crates/oxide-app/src/library/editor/supply/labelled_input.md) |
| called_by | [listings_section](/crates/oxide-app/src/library/editor/supply/listings_section.md) |
| called_by | [primary_form](/crates/oxide-app/src/library/editor/supply/primary_form.md) |
| called_by | [section_header](/crates/oxide-app/src/library/editor/supply/section_header.md) |
| called_by | [view](/crates/oxide-app/src/library/new_component/view.md) |
| called_by | [view](/crates/oxide-app/src/library/panel/view.md) |
| called_by | [close_x](/crates/oxide-app/src/library/picker/close_x.md) |
| called_by | [view](/crates/oxide-app/src/library/picker/view.md) |
| called_by | [close_x](/crates/oxide-app/src/library/primitive_picker/close_x.md) |
| called_by | [view](/crates/oxide-app/src/library/primitive_picker/view.md) |
| called_by | [broken_binding_view](/crates/oxide-app/src/library/recovery/broken_binding_view.md) |
| called_by | [close_x](/crates/oxide-app/src/library/recovery/close_x.md) |
| called_by | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
| called_by | [library_missing_view](/crates/oxide-app/src/library/recovery/library_missing_view.md) |
| called_by | [view](/crates/oxide-app/src/library/settings/distributor_apis/view.md) |
| called_by | [view](/crates/oxide-app/src/library/updates_dialog/view.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| called_by | [view](/crates/oxide-app/src/panels/components_panel/mod/view.md) |
| called_by | [view_library_block](/crates/oxide-app/src/panels/components_panel/mod/view_library_block.md) |
| called_by | [view_section](/crates/oxide-app/src/panels/components_panel/mod/view_section.md) |
| called_by | [view_drc](/crates/oxide-app/src/panels/drc/view_drc.md) |
| called_by | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
| called_by | [view_footprint_library](/crates/oxide-app/src/panels/library/view_footprint_library.md) |
| called_by | [view_footprint_library_button_row](/crates/oxide-app/src/panels/library/view_footprint_library_button_row.md) |
| called_by | [view_sch_library](/crates/oxide-app/src/panels/library/view_sch_library.md) |
| called_by | [view_mcu_console](/crates/oxide-app/src/panels/mcu_console/mod/view_mcu_console.md) |
| called_by | [from_tokens](/crates/oxide-app/src/panels/palette/from_tokens.md) |
| called_by | [view_navigator](/crates/oxide-app/src/panels/projects/view_navigator.md) |
| called_by | [view_projects](/crates/oxide-app/src/panels/projects/view_projects.md) |
| called_by | [view_erc](/crates/oxide-app/src/panels/status/view_erc.md) |
| called_by | [view_messages](/crates/oxide-app/src/panels/status/view_messages.md) |
| called_by | [view_waveform](/crates/oxide-app/src/panels/waveform/mod/view_waveform.md) |
| called_by | [section_title](/crates/oxide-app/src/panels/widgets/section_title.md) |
| called_by | [view](/crates/oxide-widgets/src/active_bar/dropdown/view.md) |
| called_by | [view_button](/crates/oxide-widgets/src/active_bar/mod/view_button.md) |
| called_by | [empty_pane](/crates/oxide-widgets/src/history_pane/empty_pane.md) |
| called_by | [history_pane](/crates/oxide-widgets/src/history_pane/history_pane.md) |
| called_by | [icon_button](/crates/oxide-widgets/src/icon_button/icon_button.md) |
| called_by | [status_bar](/crates/oxide-widgets/src/status_bar/status_bar.md) |
| called_by | [render_node](/crates/oxide-widgets/src/tree_view/render_node.md) |
