---
okf_version: "0.2"
type: Function
title: view_item
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod/view_item
language: rust
---

# view_item

## Signature

```rust
fn view_item(item: ActiveBarItem<M>, tokens: &'a ThemeTokens) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M`

## Source
Lines 315–338 in `crates/oxide-widgets/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-widgets/src/active_bar/mod.md) |
| calls | [view_button](/crates/oxide-widgets/src/active_bar/mod/view_button.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view](/crates/oxide-widgets/src/active_bar/mod/view.md) |
