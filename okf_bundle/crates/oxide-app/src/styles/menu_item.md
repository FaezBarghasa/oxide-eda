---
okf_version: "0.2"
type: Function
title: menu_item
description: Menu item / popup list button — full-width hover highlight.
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/menu_item
language: rust
---

# menu_item

Menu item / popup list button — full-width hover highlight.

## Signature

```rust
pub fn menu_item(
    tokens: &ThemeTokens,
) -> impl Fn(&Theme, button::Status) -> button::Style + 'static
```

## Visibility

- `pub`

## Docstring

Menu item / popup list button — full-width hover highlight.

## Source
Lines 383–400 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [panel_list_overlay](/crates/oxide-app/src/app/view/overlays/bars/panel_list_overlay.md) |
| called_by | [view](/crates/oxide-app/src/find_replace/view.md) |
| called_by | [view_section](/crates/oxide-app/src/panels/components_panel/mod/view_section.md) |
| called_by | [view_waveform](/crates/oxide-app/src/panels/waveform/mod/view_waveform.md) |
