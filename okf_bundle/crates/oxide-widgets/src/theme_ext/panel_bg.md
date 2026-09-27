---
okf_version: "0.2"
type: Function
title: panel_bg
description: "Panel background container style (side panels, docks)."
resource: crates/oxide-widgets/src/theme_ext.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/theme_ext/panel_bg
language: rust
---

# panel_bg

Panel background container style (side panels, docks).

## Signature

```rust
pub fn panel_bg(tokens: &ThemeTokens) -> container::Style
```

## Visibility

- `pub`

## Docstring

Panel background container style (side panels, docks).

## Source
Lines 79–90 in `crates/oxide-widgets/src/theme_ext.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_ext](/crates/oxide-widgets/src/theme_ext.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| called_by | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
| called_by | [empty_pane](/crates/oxide-widgets/src/history_pane/empty_pane.md) |
| called_by | [history_pane](/crates/oxide-widgets/src/history_pane/history_pane.md) |
