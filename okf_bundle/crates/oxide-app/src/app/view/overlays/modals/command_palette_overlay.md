---
okf_version: "0.2"
type: Function
title: command_palette_overlay
description: Command palette dropdown (Ctrl+Shift+P). Painted last so it sits
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/command_palette_overlay
language: rust
---

# command_palette_overlay

Command palette dropdown (Ctrl+Shift+P). Painted last so it sits

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn command_palette_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Command palette dropdown (Ctrl+Shift+P). Painted last so it sits
above every other modal layer; click-outside dismisses via the
standard dismiss_layer pattern. Pushes the dismiss layer then the
dropdown.

## Source
Lines 469–477 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
