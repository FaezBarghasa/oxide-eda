---
okf_version: "0.2"
type: Function
title: view_action_row
resource: crates/oxide-app/src/library/browser/action_row.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/browser/action_row/view_action_row
language: rust
---

# view_action_row

## Signature

```rust
pub(super) fn view_action_row(
    library_path: &'a std::path::Path,
    table: &str,
    selected: Option<RowId>,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 7–96 in `crates/oxide-app/src/library/browser/action_row.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_row](/crates/oxide-app/src/library/browser/action_row.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
