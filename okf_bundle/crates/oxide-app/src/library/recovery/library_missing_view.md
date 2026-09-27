---
okf_version: "0.2"
type: Function
title: library_missing_view
resource: crates/oxide-app/src/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/recovery/library_missing_view
language: rust
---

# library_missing_view

## Signature

```rust
fn library_missing_view(
    path: &'a std::path::Path,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 127–196 in `crates/oxide-app/src/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/library/recovery.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [modal_footer_strip](/crates/oxide-app/src/styles/modal_footer_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [view](/crates/oxide-app/src/library/recovery/view.md) |
