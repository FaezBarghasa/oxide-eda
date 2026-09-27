---
okf_version: "0.2"
type: Function
title: detached_header
description: Borderless-window header — pressing anywhere on the header region
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets/detached_header
language: rust
---

# detached_header

Borderless-window header — pressing anywhere on the header region

## Signature

```rust
pub(crate) fn detached_header(
    header_content: Element<'a, Message>,
    modal: super::super::super::state::ModalId,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Docstring

Borderless-window header — pressing anywhere on the header region
asks iced to start an OS-level window drag. Replaces the OS title
bar for detached modals opened with `decorations: false`.

## Source
Lines 72–80 in `crates/oxide-app/src/app/view/dialogs/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/app/view/dialogs/widgets.md) |
| called_by | [view_annotate_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_dialog_body_inner.md) |
| called_by | [view_annotate_reset_confirm_body_inner](/crates/oxide-app/src/app/view/dialogs/annotate/mod/view_annotate_reset_confirm_body_inner.md) |
| called_by | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| called_by | [view_erc_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog_body_inner.md) |
| called_by | [view_print_preview_inner](/crates/oxide-app/src/app/view/print_preview/view_print_preview_inner.md) |
