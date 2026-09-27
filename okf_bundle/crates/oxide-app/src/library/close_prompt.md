---
okf_version: "0.2"
type: Module
title: close_prompt
description: "\"Close Library — Unsaved Drafts\" confirmation modal."
resource: crates/oxide-app/src/library/close_prompt.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/close_prompt
language: rust
---

# close_prompt

"Close Library — Unsaved Drafts" confirmation modal.

## Docstring

"Close Library — Unsaved Drafts" confirmation modal.

Mirrors the dock subsystem's `ProjectCloseConfirm` flow (see
`app/view/dialogs.rs::view_project_close_confirm_body`) — when
the user closes a library that still has at least one editor
window with `dirty = true`, the dispatcher diverts to
[`crate::library::messages::LibraryMessage::ConfirmCloseLibrary`]
and shows this modal listing every dirty draft so the user can
Save All / Discard All / Cancel.

Reuses the picker modal's chrome (`modal_card` /
`modal_header_strip` / `modal_footer_strip`) for visual parity
across the Library subsystem.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/close_prompt/view.md) |
| related | [secondary_btn](/crates/oxide-app/src/library/close_prompt/secondary_btn.md) |
| related | [primary_btn](/crates/oxide-app/src/library/close_prompt/primary_btn.md) |
| related | [close_x](/crates/oxide-app/src/library/close_prompt/close_x.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
