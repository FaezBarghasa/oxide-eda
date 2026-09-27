---
okf_version: "0.2"
type: Function
title: view
description: Render the bar.
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod/view
language: rust
---

# view

Render the bar.

## Signature

```rust
pub fn view(items: Vec<ActiveBarItem<M>>, tokens: &'a ThemeTokens) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M`

## Visibility

- `pub`

## Docstring

Render the bar.

Returns an `Element<M>` ready to push into a `Stack` overlay
layer. The bar is `Length::Shrink` width-wise so the caller
controls horizontal positioning (centred via a parent
`container.align_x(Center)`, etc.).

## Source
Lines 284–313 in `crates/oxide-widgets/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-widgets/src/active_bar/mod.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [view_item](/crates/oxide-widgets/src/active_bar/mod/view_item.md) |
| called_by | [view_with_overlay](/crates/oxide-widgets/src/active_bar/mod/view_with_overlay.md) |
