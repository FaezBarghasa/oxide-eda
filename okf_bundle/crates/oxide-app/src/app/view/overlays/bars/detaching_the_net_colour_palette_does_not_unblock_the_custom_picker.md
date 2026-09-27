---
okf_version: "0.2"
type: Function
title: detaching_the_net_colour_palette_does_not_unblock_the_custom_picker
description: "The filter is one term, not four. `net_color_custom` is the"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/detaching_the_net_colour_palette_does_not_unblock_the_custom_picker
language: rust
---

# detaching_the_net_colour_palette_does_not_unblock_the_custom_picker

The filter is one term, not four. `net_color_custom` is the

## Signature

```rust
fn detaching_the_net_colour_palette_does_not_unblock_the_custom_picker()
```

## Decorators

- `test`

## Docstring

The filter is one term, not four. `net_color_custom` is the
bespoke picker, a different overlay from the detachable F5 palette
(`ModalId::NetColorPalette`) — detaching that palette must not
unblock the picker.
[test]

## Source
Lines 795–806 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
