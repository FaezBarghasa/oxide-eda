---
okf_version: "0.2"
type: Function
title: view_net_color_palette_body
description: Altium F5 Net Color palette — list of net labels with a per-net
resource: crates/oxide-app/src/app/view/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/modals/view_net_color_palette_body_1
language: rust
---

# view_net_color_palette_body

Altium F5 Net Color palette — list of net labels with a per-net

## Signature

```rust
pub(super) fn view_net_color_palette_body(&self) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Docstring

Altium F5 Net Color palette — list of net labels with a per-net
color picker. Ships with a 10-swatch palette; a full ColorPicker
widget can replace it later without changing the message contract.

## Source
Lines 169–327 in `crates/oxide-app/src/app/view/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/modals.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
