---
okf_version: "0.2"
type: Function
title: footprint_active_bar_overlay
description: v0.13 — footprint editor active bar (+ its dropdown overlay)
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/footprint_active_bar_overlay_1
language: rust
---

# footprint_active_bar_overlay

v0.13 — footprint editor active bar (+ its dropdown overlay)

## Signature

```rust
pub(in crate::app::view) fn footprint_active_bar_overlay(&self) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.13 — footprint editor active bar (+ its dropdown overlay)
mounted at the SAME app-view layer as the schematic's, so both
share identical `Space::height(y_offset + 4.0)` math and land on
a pixel-identical screen y.

## Source
Lines 232–281 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
