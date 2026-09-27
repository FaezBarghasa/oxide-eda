---
okf_version: "0.2"
type: Function
title: grid_picker_overlay
description: v0.18.10 — Altium-style grid picker popup. Floats at the cursor
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/grid_picker_overlay_1
language: rust
---

# grid_picker_overlay

v0.18.10 — Altium-style grid picker popup. Floats at the cursor

## Signature

```rust
pub(super) fn grid_picker_overlay(&self) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(super)`

## Docstring

v0.18.10 — Altium-style grid picker popup. Floats at the cursor
when `G` is pressed in a footprint editor. Pushes dismiss then the
clamped menu.

## Source
Lines 590–626 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |
