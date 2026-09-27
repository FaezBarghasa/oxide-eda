---
okf_version: "0.2"
type: Function
title: text_primary
description: Primary text color from theme tokens.
resource: crates/oxide-widgets/src/theme_ext.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/theme_ext/text_primary
language: rust
---

# text_primary

Primary text color from theme tokens.

## Signature

```rust
pub fn text_primary(tokens: &ThemeTokens) -> Color
```

## Visibility

- `pub`

## Docstring

Primary text color from theme tokens.

## Source
Lines 24–26 in `crates/oxide-widgets/src/theme_ext.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_ext](/crates/oxide-widgets/src/theme_ext.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| called_by | [view](/crates/oxide-widgets/src/active_bar/dropdown/view.md) |
| called_by | [view_button](/crates/oxide-widgets/src/active_bar/mod/view_button.md) |
| called_by | [history_pane](/crates/oxide-widgets/src/history_pane/history_pane.md) |
| called_by | [icon_button](/crates/oxide-widgets/src/icon_button/icon_button.md) |
| called_by | [status_bar](/crates/oxide-widgets/src/status_bar/status_bar.md) |
| called_by | [render_node](/crates/oxide-widgets/src/tree_view/render_node.md) |
