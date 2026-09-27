---
okf_version: "0.2"
type: Function
title: view_params_section
resource: crates/oxide-app/src/library/edit_row_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/edit_row_modal/view_params_section
language: rust
---

# view_params_section

## Signature

```rust
fn view_params_section(
    edit: &'a EditRowModalState,
    tokens: &'a ThemeTokens,
    send: impl Fn(BrowserEditMsg) -> LibraryMessage + Clone + 'a,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 347–520 in `crates/oxide-app/src/library/edit_row_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit_row_modal](/crates/oxide-app/src/library/edit_row_modal.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view](/crates/oxide-app/src/library/edit_row_modal/view.md) |
