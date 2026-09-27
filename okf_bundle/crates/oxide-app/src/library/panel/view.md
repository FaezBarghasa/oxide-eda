---
okf_version: "0.2"
type: Function
title: view
description: Render the Library left-dock panel.
resource: crates/oxide-app/src/library/panel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/panel/view
language: rust
---

# view

Render the Library left-dock panel.

## Signature

```rust
pub fn view(state: &'a LibraryState, tokens: &'a ThemeTokens) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the Library left-dock panel.

## Source
Lines 38–145 in `crates/oxide-app/src/library/panel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panel](/crates/oxide-app/src/library/panel.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
