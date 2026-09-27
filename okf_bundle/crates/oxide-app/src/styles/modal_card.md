---
okf_version: "0.2"
type: Function
title: modal_card
description: Modal-card surface — same panel/text/border palette as
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/modal_card
language: rust
---

# modal_card

Modal-card surface — same panel/text/border palette as

## Signature

```rust
pub fn modal_card(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Modal-card surface — same panel/text/border palette as
`context_menu`, but with a wider corner radius matching the
OS chrome. Used by every modal so the rounding stays in step
with the surrounding window shell.

## Source
Lines 224–243 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_annotate_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_dialog_body_inner.md) |
| called_by | [view_annotate_reset_confirm_body_inner](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_reset_confirm_body_inner.md) |
| called_by | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| called_by | [view_app_quit_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm_body.md) |
| called_by | [view_project_close_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_project_close_confirm_body.md) |
| called_by | [view_remove_dialog_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_remove_dialog_body.md) |
| called_by | [view_erc_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog_body_inner.md) |
| called_by | [view_enable_version_control_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_enable_version_control_dialog_body.md) |
| called_by | [view_grid_properties_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_grid_properties_dialog_body.md) |
| called_by | [view_project_options_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_project_options_dialog_body.md) |
| called_by | [view_rename_dialog_body](/crates/oxide-app/src/app/view/dialogs/project/view_rename_dialog_body.md) |
| called_by | [view_selection_filter_custom_body](/crates/oxide-app/src/app/view/dialogs/project/view_selection_filter_custom_body.md) |
| called_by | [view_move_selection_body](/crates/oxide-app/src/app/view/modals/view_move_selection_body.md) |
| called_by | [view_net_color_palette_body](/crates/oxide-app/src/app/view/modals/view_net_color_palette_body.md) |
| called_by | [view_parameter_manager_body](/crates/oxide-app/src/app/view/modals/view_parameter_manager_body.md) |
| called_by | [view_print_preview_inner](/crates/oxide-app/src/app/view/print_preview/view_print_preview_inner.md) |
| called_by | [view](/crates/oxide-app/src/first_run_tour/view.md) |
| called_by | [view](/crates/oxide-app/src/keyboard_shortcuts_modal/view.md) |
| called_by | [view_empty_state](/crates/oxide-app/src/library/browser/empty_state/view_empty_state.md) |
| called_by | [view_grid](/crates/oxide-app/src/library/browser/grid/view_grid.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
| called_by | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
| called_by | [view](/crates/oxide-app/src/library/close_prompt/view.md) |
| called_by | [view](/crates/oxide-app/src/library/create_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/document_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/edit_row_modal/view.md) |
| called_by | [view_delete_confirm](/crates/oxide-app/src/library/edit_row_modal/view_delete_confirm.md) |
| called_by | [view_pinned_input](/crates/oxide-app/src/library/editor/datasheet_picker/view_pinned_input.md) |
| called_by | [view_align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal/view_align_modal.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/body3d/view.md) |
| called_by | [view_move_by_modal](/crates/oxide-app/src/library/editor/footprint/move_by_modal/view_move_by_modal.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/step_attach/view.md) |
| called_by | [placeholder_card](/crates/oxide-app/src/library/editor/mod/placeholder_card.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/params/view.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/preview/view.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/sim/mod/view.md) |
| called_by | [view_symbol](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/supply/view.md) |
| called_by | [view](/crates/oxide-app/src/library/new_component/view.md) |
| called_by | [view](/crates/oxide-app/src/library/picker/view.md) |
| called_by | [view](/crates/oxide-app/src/library/primitive_picker/view.md) |
| called_by | [broken_binding_view](/crates/oxide-app/src/library/recovery/broken_binding_view.md) |
| called_by | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
| called_by | [library_missing_view](/crates/oxide-app/src/library/recovery/library_missing_view.md) |
| called_by | [view](/crates/oxide-app/src/library/settings/distributor_apis/view.md) |
| called_by | [view](/crates/oxide-app/src/library/updates_dialog/view.md) |
| called_by | [view](/crates/oxide-app/src/passive_calculator_modal/view.md) |
