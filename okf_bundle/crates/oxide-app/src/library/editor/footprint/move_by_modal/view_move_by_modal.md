---
okf_version: "0.2"
type: Function
title: view_move_by_modal
description: Build the Move-By modal card for the active footprint editor.
resource: crates/oxide-app/src/library/editor/footprint/move_by_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/move_by_modal/view_move_by_modal
language: rust
---

# view_move_by_modal

Build the Move-By modal card for the active footprint editor.

## Signature

```rust
pub fn view_move_by_modal(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> Option<Element<'a, LibraryMessage>>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the Move-By modal card for the active footprint editor.
Returns `None` when the modal is closed (`move_by_modal` is
`None`), so the call site can no-op cleanly.

## Source
Lines 37–175 in `crates/oxide-app/src/library/editor/footprint/move_by_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [move_by_modal](/crates/oxide-app/src/library/editor/footprint/move_by_modal.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [footprint_move_by_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_move_by_overlay.md) |
