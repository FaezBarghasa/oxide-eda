---
okf_version: "0.2"
type: Function
title: view
description: Render the Library Browser tab body. Returns an empty-state panel
resource: crates/oxide-app/src/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/mod/view
language: rust
---

# view

Render the Library Browser tab body. Returns an empty-state panel

## Signature

```rust
pub fn view(
    library_path: &'a std::path::Path,
    library_state: &'a LibraryState,
    browser: &'a LibraryBrowserState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the Library Browser tab body. Returns an empty-state panel
when the library isn't currently mounted (e.g. mount failed) so the
tab still renders without panicking.

## Source
Lines 73–227 in `crates/oxide-app/src/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/library/browser/mod.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| calls | [view_empty_state](/crates/oxide-app/src/library/browser/empty_state/view_empty_state.md) |
| calls | [view_table_sidebar](/crates/oxide-app/src/library/browser/sidebar/view_table_sidebar.md) |
| calls | [row_matches_filter](/crates/oxide-app/src/library/browser/columns/row_matches_filter.md) |
| calls | [derive_columns](/crates/oxide-app/src/library/browser/columns/derive_columns.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [compare_cells](/crates/oxide-app/src/library/browser/columns/compare_cells.md) |
| calls | [view_grid](/crates/oxide-app/src/library/browser/grid/view_grid.md) |
| calls | [view_action_row](/crates/oxide-app/src/library/browser/action_row/view_action_row.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
