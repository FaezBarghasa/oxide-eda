---
okf_version: "0.2"
type: Function
title: view
description: "Render the dropdown panel as an `Element<M>`. `width_hint`"
resource: crates/oxide-widgets/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/active_bar/dropdown/view
language: rust
---

# view

Render the dropdown panel as an `Element<M>`. `width_hint`

## Signature

```rust
pub fn view(
    entries: Vec<DropdownEntry<M>>,
    tokens: &'a ThemeTokens,
    width_hint: Option<f32>,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M`

## Visibility

- `pub`

## Docstring

Render the dropdown panel as an `Element<M>`. `width_hint`
specifies a fixed panel width in px (e.g. 220) when the menu is
list-style; `None` lets the panel auto-size (used for the Filter
chip-grid that drives its own width). Caller wraps the result in a
Translate / Stack overlay layer at the chevron's anchor and pairs
it with a transparent backstop for click-outside-to-dismiss.

## Source
Lines 139–280 in `crates/oxide-widgets/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-widgets/src/active_bar/dropdown.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [text_primary](/crates/oxide-widgets/src/theme_ext/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
