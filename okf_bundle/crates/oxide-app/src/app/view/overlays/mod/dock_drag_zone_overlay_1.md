---
okf_version: "0.2"
type: Function
title: dock_drag_zone_overlay
description: Dock drag-target highlight — paints the left / right / bottom dock
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/dock_drag_zone_overlay_1
language: rust
---

# dock_drag_zone_overlay

Dock drag-target highlight — paints the left / right / bottom dock

## Signature

```rust
pub(super) fn dock_drag_zone_overlay(&self) -> Option<Element<'_, Message>>
```

## Visibility

- `pub(super)`

## Docstring

Dock drag-target highlight — paints the left / right / bottom dock
zone the currently-dragged floating panel would snap into.

## Source
Lines 630–678 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [dock_zone_highlight](/crates/oxide-app/src/styles/dock_zone_highlight.md) |
