---
okf_version: "0.2"
type: Function
title: view_selection_filter_custom_dialog
description: v0.18.14.1 — Custom Selection Filter modal. 8 rows of
resource: crates/oxide-app/src/app/view/dialogs/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/project/view_selection_filter_custom_dialog_1
language: rust
---

# view_selection_filter_custom_dialog

v0.18.14.1 — Custom Selection Filter modal. 8 rows of

## Signature

```rust
pub(in crate::app::view) fn view_selection_filter_custom_dialog(&self) -> Element<'_, Message>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.18.14.1 — Custom Selection Filter modal. 8 rows of
per-kind checkboxes (Pads / Tracks / Arcs / Pours / 3D Bodies
/ Keepouts / Cutouts / Texts) + Apply / Cancel. Apply writes
the draft into `editor.state.selection_filter`.

## Source
Lines 621–630 in `crates/oxide-app/src/app/view/dialogs/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/src/app/view/dialogs/project.md) |
| calls | [wrap_modal](/crates/oxide-app/src/app/view/dialogs/widgets/wrap_modal.md) |
