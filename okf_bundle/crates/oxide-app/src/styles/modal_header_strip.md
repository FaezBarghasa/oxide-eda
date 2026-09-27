---
okf_version: "0.2"
type: Function
title: modal_header_strip
description: "Modal header strip — same toolbar bg as `toolbar_strip` but with a"
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/modal_header_strip
language: rust
---

# modal_header_strip

Modal header strip — same toolbar bg as `toolbar_strip` but with a

## Signature

```rust
pub fn modal_header_strip(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Modal header strip — same toolbar bg as `toolbar_strip` but with a
top-only rounded radius matching `MODAL_CORNER_RADIUS`. Without
this, the modal's outer 8 px rounded border is visually masked by
the header's own rectangular background filling into the corners
(iced's `Container::clip(true)` clips to the bounds rectangle, not
the rounded path).

## Source
Lines 72–88 in `crates/oxide-app/src/styles.rs`

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
| called_by | [view](/crates/oxide-app/src/library/close_prompt/view.md) |
| called_by | [view](/crates/oxide-app/src/library/create_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/document_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/edit_row_modal/view.md) |
| called_by | [view_delete_confirm](/crates/oxide-app/src/library/edit_row_modal/view_delete_confirm.md) |
| called_by | [view_header](/crates/oxide-app/src/library/editor/mod/view_header.md) |
| called_by | [view](/crates/oxide-app/src/library/new_component/view.md) |
| called_by | [view](/crates/oxide-app/src/library/picker/view.md) |
| called_by | [view](/crates/oxide-app/src/library/primitive_picker/view.md) |
| called_by | [broken_binding_view](/crates/oxide-app/src/library/recovery/broken_binding_view.md) |
| called_by | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
| called_by | [library_missing_view](/crates/oxide-app/src/library/recovery/library_missing_view.md) |
| called_by | [view](/crates/oxide-app/src/library/updates_dialog/view.md) |
| called_by | [view](/crates/oxide-app/src/passive_calculator_modal/view.md) |
