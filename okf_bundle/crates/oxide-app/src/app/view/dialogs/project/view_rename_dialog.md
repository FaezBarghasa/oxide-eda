---
okf_version: "0.2"
type: Function
title: view_rename_dialog
description: ────────────────────────────────────────────────────────────────
resource: crates/oxide-app/src/app/view/dialogs/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/project/view_rename_dialog
language: rust
---

# view_rename_dialog

────────────────────────────────────────────────────────────────

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn view_rename_dialog(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

────────────────────────────────────────────────────────────────
Small Altium-style modals (Rename / Remove-from-Project / Close-
Tab Confirm). All three render through the same chrome used by
Annotate / ERC: `draggable_header` + `close_x_button` on the
right + `wrap_modal` for absolute positioning with drag-offset
persistence in `ui_state.modal_offsets`.
────────────────────────────────────────────────────────────────

## Source
Lines 26–35 in `crates/oxide-app/src/app/view/dialogs/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/src/app/view/dialogs/project.md) |
| calls | [wrap_modal](/crates/oxide-app/src/app/view/dialogs/widgets/wrap_modal.md) |
