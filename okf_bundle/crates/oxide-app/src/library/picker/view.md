---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/library/picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/picker/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    state: &'a LibraryState,
    picker: &'a PickerState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 32–200 in `crates/oxide-app/src/library/picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [picker](/crates/oxide-app/src/library/picker.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [list_components_filtered](/crates/oxide-app/src/library/commands/list_components_filtered.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_footer_strip](/crates/oxide-app/src/styles/modal_footer_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
