---
okf_version: "0.2"
type: Function
title: modal_footer_strip
description: "Modal footer strip — mirrors `modal_header_strip` for the bottom"
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/modal_footer_strip
language: rust
---

# modal_footer_strip

Modal footer strip — mirrors `modal_header_strip` for the bottom

## Signature

```rust
pub fn modal_footer_strip(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Modal footer strip — mirrors `modal_header_strip` for the bottom
of a modal: same toolbar bg, BL+BR rounded radius matching
`MODAL_CORNER_RADIUS`. Apply to the very bottom container of a
modal that's wider than the body padding (button rows, status
rows) so the rectangular bg doesn't paint into the modal's
rounded bottom corners.

## Source
Lines 96–112 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| called_by | [view](/crates/oxide-app/src/library/close_prompt/view.md) |
| called_by | [view](/crates/oxide-app/src/library/create_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/document_options/view.md) |
| called_by | [view](/crates/oxide-app/src/library/edit_row_modal/view.md) |
| called_by | [view_delete_confirm](/crates/oxide-app/src/library/edit_row_modal/view_delete_confirm.md) |
| called_by | [view_footer](/crates/oxide-app/src/library/editor/mod/view_footer.md) |
| called_by | [view_footprint_footer](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_footer.md) |
| called_by | [view](/crates/oxide-app/src/library/new_component/view.md) |
| called_by | [view](/crates/oxide-app/src/library/picker/view.md) |
| called_by | [view](/crates/oxide-app/src/library/primitive_picker/view.md) |
| called_by | [broken_binding_view](/crates/oxide-app/src/library/recovery/broken_binding_view.md) |
| called_by | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
| called_by | [library_missing_view](/crates/oxide-app/src/library/recovery/library_missing_view.md) |
| called_by | [view](/crates/oxide-app/src/library/updates_dialog/view.md) |
