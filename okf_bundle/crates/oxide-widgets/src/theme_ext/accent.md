---
okf_version: "0.2"
type: Function
title: accent
description: "Accent color (for highlights, active elements)."
resource: crates/oxide-widgets/src/theme_ext.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/theme_ext/accent
language: rust
---

# accent

Accent color (for highlights, active elements).

## Signature

```rust
pub fn accent(tokens: &ThemeTokens) -> Color
```

## Visibility

- `pub`

## Docstring

Accent color (for highlights, active elements).

## Source
Lines 34–36 in `crates/oxide-widgets/src/theme_ext.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_ext](/crates/oxide-widgets/src/theme_ext.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| called_by | [view_copilot](/crates/oxide-app/src/panels/copilot/view_copilot.md) |
| called_by | [view_mcu_console](/crates/oxide-app/src/panels/mcu_console/mod/view_mcu_console.md) |
| called_by | [view_erc](/crates/oxide-app/src/panels/status/view_erc.md) |
| called_by | [view_messages](/crates/oxide-app/src/panels/status/view_messages.md) |
| called_by | [icon_button](/crates/oxide-widgets/src/icon_button/icon_button.md) |
