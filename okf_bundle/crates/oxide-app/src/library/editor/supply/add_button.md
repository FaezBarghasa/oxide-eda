---
okf_version: "0.2"
type: Function
title: add_button
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/add_button
language: rust
---

# add_button

## Signature

```rust
fn add_button(
    label: &'static str,
    msg: EditorMsg,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 533–569 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [alternates_section](/crates/oxide-app/src/library/editor/supply/alternates_section.md) |
