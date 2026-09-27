---
okf_version: "0.2"
type: Function
title: icon_button
description: Create a themed toolbar icon button.
resource: crates/oxide-widgets/src/icon_button.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/icon_button/icon_button
language: rust
---

# icon_button

Create a themed toolbar icon button.

## Signature

```rust
pub fn icon_button(
    icon: &str,
    tooltip_text: &str,
    on_press: M,
    active: bool,
    tokens: &ThemeTokens,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: Clone + 'a`

## Visibility

- `pub`

## Docstring

Create a themed toolbar icon button.

* `icon`         — Unicode char or short text displayed on the button face.
* `tooltip_text` — Text shown on hover in a tooltip.
* `on_press`     — Message emitted when the button is pressed.
* `active`       — Whether this button is in the "active" / pressed state.
* `tokens`       — Theme tokens for colors.

## Source
Lines 23–65 in `crates/oxide-widgets/src/icon_button.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [icon_button](/crates/oxide-widgets/src/icon_button.md) |
| calls | [text_primary](/crates/oxide-widgets/src/theme_ext/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [accent](/crates/oxide-widgets/src/theme_ext/accent.md) |
