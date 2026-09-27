---
okf_version: "0.2"
type: Function
title: view_align_modal
description: Build the Align dialog card for the active footprint editor. Returns
resource: crates/oxide-app/src/library/editor/footprint/align_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/align_modal/view_align_modal
language: rust
---

# view_align_modal

Build the Align dialog card for the active footprint editor. Returns

## Signature

```rust
pub fn view_align_modal(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> Option<Element<'a, LibraryMessage>>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the Align dialog card for the active footprint editor. Returns
`None` when the modal is closed (`align_modal` is `None`), so the
call site can no-op cleanly.

## Source
Lines 63–173 in `crates/oxide-app/src/library/editor/footprint/align_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [option_chip](/crates/oxide-app/src/library/editor/footprint/align_modal/option_chip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [footprint_align_overlay](/crates/oxide-app/src/app/view/overlays/bars/footprint_align_overlay.md) |
