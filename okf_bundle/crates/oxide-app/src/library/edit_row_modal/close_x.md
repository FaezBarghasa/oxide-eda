---
okf_version: "0.2"
type: Function
title: close_x
resource: crates/oxide-app/src/library/edit_row_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/edit_row_modal/close_x
language: rust
---

# close_x

## Signature

```rust
fn close_x(
    library_path: &'a std::path::Path,
    tokens: &ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 522–555 in `crates/oxide-app/src/library/edit_row_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit_row_modal](/crates/oxide-app/src/library/edit_row_modal.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
