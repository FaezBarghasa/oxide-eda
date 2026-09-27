---
okf_version: "0.2"
type: Function
title: selection_color
description: Selection highlight background.
resource: crates/oxide-widgets/src/theme_ext.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/theme_ext/selection_color
language: rust
---

# selection_color

Selection highlight background.

## Signature

```rust
pub fn selection_color(tokens: &ThemeTokens) -> Color
```

## Visibility

- `pub`

## Docstring

Selection highlight background.

## Source
Lines 59–61 in `crates/oxide-widgets/src/theme_ext.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_ext](/crates/oxide-widgets/src/theme_ext.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| called_by | [view_erc](/crates/oxide-app/src/panels/status/view_erc.md) |
| called_by | [render_node](/crates/oxide-widgets/src/tree_view/render_node.md) |
