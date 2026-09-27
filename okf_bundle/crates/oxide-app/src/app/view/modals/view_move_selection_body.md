---
okf_version: "0.2"
type: Function
title: view_move_selection_body
description: Altium-style Move Selection dialog. Two numeric inputs plus
resource: crates/oxide-app/src/app/view/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/modals/view_move_selection_body
language: rust
---

# view_move_selection_body

Altium-style Move Selection dialog. Two numeric inputs plus

## Signature

```rust
impl Oxide { pub(super) fn view_move_selection_body(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Altium-style Move Selection dialog. Two numeric inputs plus
OK / Cancel. No header drag region on the body itself — the
modal opens borderless so the OS-window-drag handler owns that.

## Source
Lines 15–131 in `crates/oxide-app/src/app/view/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/modals.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
