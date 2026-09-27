---
okf_version: "0.2"
type: Function
title: net_color_custom_overlay
description: Custom net-colour picker. Bespoke modal (not the iced_aw
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/net_color_custom_overlay_1
language: rust
---

# net_color_custom_overlay

Custom net-colour picker. Bespoke modal (not the iced_aw

## Signature

```rust
pub(in crate::app::view) fn net_color_custom_overlay(&self) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Custom net-colour picker. Bespoke modal (not the iced_aw
ColorPicker) because the user needs a quick-pick palette +
precise RGB inputs side-by-side. Pushes the dismiss backdrop
then the picker card.

## Source
Lines 110–118 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
