---
okf_version: "0.2"
type: Function
title: view_net_color_custom_picker
description: Custom net-colour picker modal. Grid of quick-pick swatches on
resource: crates/oxide-app/src/app/view/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/modals/view_net_color_custom_picker
language: rust
---

# view_net_color_custom_picker

Custom net-colour picker modal. Grid of quick-pick swatches on

## Signature

```rust
impl Oxide { pub(super) fn view_net_color_custom_picker(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Custom net-colour picker modal. Grid of quick-pick swatches on
the left, precise R / G / B / hex on the right, live preview
and OK / Cancel at the bottom. Ships with a 24-color palette
matching the common Altium net-colour presets plus a handful of
EDA-specific diagnostic colours.

## Source
Lines 489–748 in `crates/oxide-app/src/app/view/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/modals.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [context_menu](/crates/oxide-app/src/styles/context_menu.md) |
