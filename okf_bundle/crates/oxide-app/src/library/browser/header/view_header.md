---
okf_version: "0.2"
type: Function
title: view_header
resource: crates/oxide-app/src/library/browser/header.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/browser/header/view_header
language: rust
---

# view_header

## Signature

```rust
pub(super) fn view_header(
    library_path: &'a std::path::Path,
    _lib: &'a OpenLibrary,
    browser: &'a LibraryBrowserState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 9–84 in `crates/oxide-app/src/library/browser/header.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [header](/crates/oxide-app/src/library/browser/header.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [tab_bar_strip](/crates/oxide-app/src/styles/tab_bar_strip.md) |
