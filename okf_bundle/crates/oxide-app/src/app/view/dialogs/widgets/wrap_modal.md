---
okf_version: "0.2"
type: Function
title: wrap_modal
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets/wrap_modal
language: rust
---

# wrap_modal

## Signature

```rust
pub(in crate::app::view) fn wrap_modal(
    inner: Element<'a, Message>,
    offset: (f32, f32),
    window_size: (f32, f32),
    modal_size: (f32, f32),
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::app::view)`

## Source
Lines 16–51 in `crates/oxide-app/src/app/view/dialogs/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/app/view/dialogs/widgets.md) |
| called_by | [view_annotate_dialog](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_dialog.md) |
| called_by | [view_annotate_reset_confirm](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_reset_confirm.md) |
| called_by | [view_bom_preview](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview.md) |
| called_by | [view_app_quit_confirm](/crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm.md) |
| called_by | [view_project_close_confirm](/crates/oxide-app/src/app/view/dialogs/confirms/view_project_close_confirm.md) |
| called_by | [view_remove_dialog](/crates/oxide-app/src/app/view/dialogs/confirms/view_remove_dialog.md) |
| called_by | [view_erc_dialog](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog.md) |
| called_by | [view_enable_version_control_dialog](/crates/oxide-app/src/app/view/dialogs/project/view_enable_version_control_dialog.md) |
| called_by | [view_grid_properties_dialog](/crates/oxide-app/src/app/view/dialogs/project/view_grid_properties_dialog.md) |
| called_by | [view_project_options_dialog](/crates/oxide-app/src/app/view/dialogs/project/view_project_options_dialog.md) |
| called_by | [view_rename_dialog](/crates/oxide-app/src/app/view/dialogs/project/view_rename_dialog.md) |
| called_by | [view_selection_filter_custom_dialog](/crates/oxide-app/src/app/view/dialogs/project/view_selection_filter_custom_dialog.md) |
| called_by | [view_print_preview](/crates/oxide-app/src/app/view/print_preview/view_print_preview.md) |
