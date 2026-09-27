---
okf_version: "0.2"
type: Function
title: view_table_sidebar
description: Vertical table sidebar — replaces the old horizontal tab strip
resource: crates/oxide-app/src/library/browser/sidebar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/browser/sidebar/view_table_sidebar
language: rust
---

# view_table_sidebar

Vertical table sidebar — replaces the old horizontal tab strip

## Signature

```rust
pub(super) fn view_table_sidebar(
    library_path: &'a std::path::Path,
    library_state: &'a LibraryState,
    lib: &'a OpenLibrary,
    browser: &'a LibraryBrowserState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Vertical table sidebar — replaces the old horizontal tab strip
with a database-style master pane: each table is one row, the
active one highlights, and `+ Table` (plus the inline create
form) anchors at the bottom. Per-tab × delete still ships with
the next iteration; for now an empty table is selectable and the
user can drop rows individually before deletion lands.

## Source
Lines 17–740 in `crates/oxide-app/src/library/browser/sidebar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sidebar](/crates/oxide-app/src/library/browser/sidebar.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
