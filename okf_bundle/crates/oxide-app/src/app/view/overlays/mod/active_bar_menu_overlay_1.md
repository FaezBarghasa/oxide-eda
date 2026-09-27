---
okf_version: "0.2"
type: Function
title: active_bar_menu_overlay
description: Active-bar dropdown menu (Place / Align / filter presets etc.).
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/active_bar_menu_overlay_1
language: rust
---

# active_bar_menu_overlay

Active-bar dropdown menu (Place / Align / filter presets etc.).

## Signature

```rust
pub(super) fn active_bar_menu_overlay(&self) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(super)`

## Docstring

Active-bar dropdown menu (Place / Align / filter presets etc.).
Absolute-positioned with `Translate` so the column can auto-size
to its widest label. Pushes the dismiss layer then the dropdown.

## Source
Lines 317–352 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |
| calls | [view_dropdown](/crates/oxide-app/src/active_bar/dropdown/view_dropdown.md) |
| calls | [dropdown_x_offset](/crates/oxide-app/src/active_bar/dropdown/dropdown_x_offset.md) |
