---
okf_version: "0.2"
type: Function
title: view_empty_state
resource: crates/oxide-app/src/library/browser/empty_state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/browser/empty_state/view_empty_state
language: rust
---

# view_empty_state

## Signature

```rust
pub(super) fn view_empty_state(
    library_path: &'a std::path::Path,
    lib: &'a OpenLibrary,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 9–77 in `crates/oxide-app/src/library/browser/empty_state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [empty_state](/crates/oxide-app/src/library/browser/empty_state.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
