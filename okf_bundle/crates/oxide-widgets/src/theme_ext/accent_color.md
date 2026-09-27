---
okf_version: "0.2"
type: Function
title: accent_color
description: Theme accent color — used for active-project markers and the
resource: crates/oxide-widgets/src/theme_ext.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/theme_ext/accent_color
language: rust
---

# accent_color

Theme accent color — used for active-project markers and the

## Signature

```rust
pub fn accent_color(tokens: &ThemeTokens) -> Color
```

## Visibility

- `pub`

## Docstring

Theme accent color — used for active-project markers and the
"open" indicator dot on the tree.

## Source
Lines 70–72 in `crates/oxide-widgets/src/theme_ext.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_ext](/crates/oxide-widgets/src/theme_ext.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| called_by | [render_node](/crates/oxide-widgets/src/tree_view/render_node.md) |
