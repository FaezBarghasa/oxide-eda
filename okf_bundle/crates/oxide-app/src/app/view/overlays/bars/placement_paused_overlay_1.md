---
okf_version: "0.2"
type: Function
title: placement_paused_overlay
description: "Altium-style pause overlay: big centered \"Placement Paused\" card"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/placement_paused_overlay_1
language: rust
---

# placement_paused_overlay

Altium-style pause overlay: big centered "Placement Paused" card

## Signature

```rust
pub(in crate::app::view) fn placement_paused_overlay(&self) -> Option<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Altium-style pause overlay: big centered "Placement Paused" card
with a Resume button. Clicking Resume clears `pre_placement`,
un-pauses the canvas, and drops back to the active placement tool
so the user can keep dropping objects with the edited properties.
v0.13 — Also fires when a footprint editor's placement is paused
so TAB during pad/via/string placement surfaces the same overlay.

## Source
Lines 126–190 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
