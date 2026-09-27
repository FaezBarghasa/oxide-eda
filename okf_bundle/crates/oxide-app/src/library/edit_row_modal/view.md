---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/library/edit_row_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/edit_row_modal/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    library_path: &'a std::path::Path,
    edit: &'a EditRowModalState,
    tokens: &'a ThemeTokens,
    classes: Vec<crate::fonts::ComponentClassEntry>,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 56–345 in `crates/oxide-app/src/library/edit_row_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit_row_modal](/crates/oxide-app/src/library/edit_row_modal.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [short_uuid](/crates/oxide-app/src/library/edit_row_modal/short_uuid.md) |
| calls | [view_params_section](/crates/oxide-app/src/library/edit_row_modal/view_params_section.md) |
| calls | [modal_footer_strip](/crates/oxide-app/src/styles/modal_footer_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
