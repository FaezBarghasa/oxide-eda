---
okf_version: "0.2"
type: Function
title: draggable_header
description: Wrap a header element in a mouse_area so pressing on it begins a modal
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets/draggable_header
language: rust
---

# draggable_header

Wrap a header element in a mouse_area so pressing on it begins a modal

## Signature

```rust
pub(in crate::app::view) fn draggable_header(
    header_content: Element<'a, Message>,
    modal: super::super::super::state::ModalId,
    last_mouse: (f32, f32),
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::app::view)`

## Docstring

Wrap a header element in a mouse_area so pressing on it begins a modal
drag. Uses the last known mouse position as the drag anchor.

## Source
Lines 55–67 in `crates/oxide-app/src/app/view/dialogs/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/app/view/dialogs/widgets.md) |
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
| called_by | [view_print_preview_inner](/crates/oxide-app/src/app/view/print_preview/view_print_preview_inner.md) |
