---
okf_version: "0.2"
type: Function
title: footprint_context_menu_overlay
description: v0.26 — right-click context menu overlay for the footprint
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/footprint_context_menu_overlay
language: rust
---

# footprint_context_menu_overlay

v0.26 — right-click context menu overlay for the footprint

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn footprint_context_menu_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.26 — right-click context menu overlay for the footprint
canvas. Sits above the active-bar dropdown so a long-press menu
is occluded by — never under — its own dismiss layer. Pushes the
dismiss layer then the clamped menu card.

## Source
Lines 287–344 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
