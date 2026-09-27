---
okf_version: "0.2"
type: Function
title: view_delete_confirm
description: Delete-row confirm modal — sibling overlay launched from the
resource: crates/oxide-app/src/library/edit_row_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/edit_row_modal/view_delete_confirm
language: rust
---

# view_delete_confirm

Delete-row confirm modal — sibling overlay launched from the

## Signature

```rust
pub fn view_delete_confirm(
    library_path: &'a std::path::Path,
    confirm: &'a DeleteConfirmState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Delete-row confirm modal — sibling overlay launched from the
Library Browser action row's Delete Selected button.

## Source
Lines 564–656 in `crates/oxide-app/src/library/edit_row_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit_row_modal](/crates/oxide-app/src/library/edit_row_modal.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [modal_footer_strip](/crates/oxide-app/src/styles/modal_footer_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [delete_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/delete_confirm_overlay.md) |
