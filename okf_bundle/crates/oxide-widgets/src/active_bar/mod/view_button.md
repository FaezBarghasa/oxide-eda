---
okf_version: "0.2"
type: Function
title: view_button
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod/view_button
language: rust
---

# view_button

## Signature

```rust
fn view_button(b: ActiveBarButton<M>, tokens: &'a ThemeTokens) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M`

## Source
Lines 340–500 in `crates/oxide-widgets/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-widgets/src/active_bar/mod.md) |
| calls | [text_primary](/crates/oxide-widgets/src/theme_ext/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [hover_color](/crates/oxide-widgets/src/theme_ext/hover_color.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
| called_by | [view_item](/crates/oxide-widgets/src/active_bar/mod/view_item.md) |
