---
okf_version: "0.2"
type: Module
title: confirms
description: "Confirmation modals — Remove-from-Project, Close-Project confirm, and"
resource: crates/oxide-app/src/app/view/dialogs/confirms.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/confirms
language: rust
---

# confirms

Confirmation modals — Remove-from-Project, Close-Project confirm, and

## Docstring

Confirmation modals — Remove-from-Project, Close-Project confirm, and
Quit-app confirm.

Extracted verbatim from `view/dialogs.rs` (ADR-0001, issue #164) as pure
code motion — no behaviour change. These are methods of the same
`Oxide` view impl, split across sibling files.

## Relationships

| Type | Target |
|------|--------|
| related | [view_remove_dialog](/crates/oxide-app/src/app/view/dialogs/confirms/view_remove_dialog.md) |
| related | [view_remove_dialog_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_remove_dialog_body.md) |
| related | [view_project_close_confirm](/crates/oxide-app/src/app/view/dialogs/confirms/view_project_close_confirm.md) |
| related | [view_project_close_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_project_close_confirm_body.md) |
| related | [view_app_quit_confirm](/crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm.md) |
| related | [view_app_quit_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm_body.md) |
| related | [view_remove_dialog](/crates/oxide-app/src/app/view/dialogs/confirms/view_remove_dialog.md) |
| related | [view_remove_dialog_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_remove_dialog_body.md) |
| related | [view_project_close_confirm](/crates/oxide-app/src/app/view/dialogs/confirms/view_project_close_confirm.md) |
| related | [view_project_close_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_project_close_confirm_body.md) |
| related | [view_app_quit_confirm](/crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm.md) |
| related | [view_app_quit_confirm_body](/crates/oxide-app/src/app/view/dialogs/confirms/view_app_quit_confirm_body.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
