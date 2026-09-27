---
okf_version: "0.2"
type: Function
title: text_primary
description: "Primary body text — the theme's guaranteed-readable text colour on the"
resource: crates/oxide-app/src/preferences/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/preferences/widgets/text_primary
language: rust
---

# text_primary

Primary body text — the theme's guaranteed-readable text colour on the

## Signature

```rust
pub(super) fn text_primary(theme: &Theme) -> text::Style
```

## Visibility

- `pub(super)`

## Docstring

Primary body text — the theme's guaranteed-readable text colour on the
base background surface.

## Source
Lines 23–27 in `crates/oxide-app/src/preferences/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/preferences/widgets.md) |
| called_by | [view_grid_picker_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_grid_picker_menu.md) |
| called_by | [view_action_row](/crates/oxide-app/src/library/browser/action_row/view_action_row.md) |
| called_by | [view_empty_state](/crates/oxide-app/src/library/browser/empty_state/view_empty_state.md) |
| called_by | [view_grid](/crates/oxide-app/src/library/browser/grid/view_grid.md) |
| called_by | [view_header](/crates/oxide-app/src/library/browser/header/view_header.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
| called_by | [preview_panel](/crates/oxide-app/src/library/browser/preview/preview_panel.md) |
| called_by | [preview_panel_with_pick](/crates/oxide-app/src/library/browser/preview/preview_panel_with_pick.md) |
| called_by | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
| called_by | [view_table_sidebar](/crates/oxide-app/src/library/browser/sidebar/view_table_sidebar.md) |
| called_by | [view](/crates/oxide-app/src/library/close_prompt/view.md) |
| called_by | [view](/crates/oxide-app/src/library/create_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/document_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/edit_row_modal/view.md) |
| called_by | [view_delete_confirm](/crates/oxide-app/src/library/edit_row_modal/view_delete_confirm.md) |
| called_by | [view_params_section](/crates/oxide-app/src/library/edit_row_modal/view_params_section.md) |
| called_by | [view_pinned_input](/crates/oxide-app/src/library/editor/datasheet_picker/view_pinned_input.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/body3d/view.md) |
| called_by | [footprint_tabs_overlay](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/footprint_tabs_overlay.md) |
| called_by | [mode_switcher_overlay](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/mode_switcher_overlay.md) |
| called_by | [build_dimension_input](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/build_dimension_input.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/step_attach/view.md) |
| called_by | [placeholder_card](/crates/oxide-app/src/library/editor/mod/placeholder_card.md) |
| called_by | [view_footer](/crates/oxide-app/src/library/editor/mod/view_footer.md) |
| called_by | [view_header](/crates/oxide-app/src/library/editor/mod/view_header.md) |
| called_by | [view_tabs](/crates/oxide-app/src/library/editor/mod/view_tabs.md) |
| called_by | [add_custom_row](/crates/oxide-app/src/library/editor/params/add_custom_row.md) |
| called_by | [custom_row](/crates/oxide-app/src/library/editor/params/custom_row.md) |
| called_by | [template_row](/crates/oxide-app/src/library/editor/params/template_row.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/params/view.md) |
| called_by | [header_row](/crates/oxide-app/src/library/editor/preview/header_row.md) |
| called_by | [open_btn](/crates/oxide-app/src/library/editor/preview/open_btn.md) |
| called_by | [override_editor_row](/crates/oxide-app/src/library/editor/preview/override_editor_row.md) |
| called_by | [pin_map_subsection](/crates/oxide-app/src/library/editor/preview/pin_map_subsection.md) |
| called_by | [render_pane](/crates/oxide-app/src/library/editor/preview/render_pane.md) |
| called_by | [pin_node_row](/crates/oxide-app/src/library/editor/sim/mod/pin_node_row.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/sim/mod/view.md) |
| called_by | [view_footprint_footer](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_footer.md) |
| called_by | [view_footprint_layers_strip](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_layers_strip.md) |
| called_by | [view_footprint_sketch_toolbar](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_sketch_toolbar.md) |
| called_by | [view_footprint_toolbar](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_toolbar.md) |
| called_by | [view_footprint_top_strip](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_top_strip.md) |
| called_by | [view_symbol_status](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_status.md) |
| called_by | [view_symbol_toolbar](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_toolbar.md) |
| called_by | [add_button](/crates/oxide-app/src/library/editor/supply/add_button.md) |
| called_by | [alternate_row](/crates/oxide-app/src/library/editor/supply/alternate_row.md) |
| called_by | [listing_row](/crates/oxide-app/src/library/editor/supply/listing_row.md) |
| called_by | [view](/crates/oxide-app/src/library/new_component/view.md) |
| called_by | [view](/crates/oxide-app/src/library/panel/view.md) |
| called_by | [view](/crates/oxide-app/src/library/picker/view.md) |
| called_by | [view](/crates/oxide-app/src/library/primitive_picker/view.md) |
| called_by | [broken_binding_view](/crates/oxide-app/src/library/recovery/broken_binding_view.md) |
| called_by | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
| called_by | [library_missing_view](/crates/oxide-app/src/library/recovery/library_missing_view.md) |
| called_by | [view](/crates/oxide-app/src/library/settings/distributor_apis/view.md) |
| called_by | [view](/crates/oxide-app/src/library/updates_dialog/view.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| called_by | [view_library_block](/crates/oxide-app/src/panels/components_panel/mod/view_library_block.md) |
| called_by | [view_section](/crates/oxide-app/src/panels/components_panel/mod/view_section.md) |
| called_by | [view_copilot](/crates/oxide-app/src/panels/copilot/view_copilot.md) |
| called_by | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
| called_by | [view_footprint_library](/crates/oxide-app/src/panels/library/view_footprint_library.md) |
| called_by | [view_footprint_library_button_row](/crates/oxide-app/src/panels/library/view_footprint_library_button_row.md) |
| called_by | [view_sch_library](/crates/oxide-app/src/panels/library/view_sch_library.md) |
| called_by | [from_tokens](/crates/oxide-app/src/panels/palette/from_tokens.md) |
| called_by | [view_messages](/crates/oxide-app/src/panels/status/view_messages.md) |
