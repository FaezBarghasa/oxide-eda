---
okf_version: "0.2"
type: Function
title: resize_edges_overlay
description: Full-window-sized Stack overlay that anchors 6 px resize hit
resource: crates/oxide-app/src/app/view/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/chrome/resize_edges_overlay_1
language: rust
---

# resize_edges_overlay

Full-window-sized Stack overlay that anchors 6 px resize hit

## Signature

```rust
pub(super) fn resize_edges_overlay() -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Full-window-sized Stack overlay that anchors 6 px resize hit
zones at the borderless main window's edges and corners. Clicks
on the edges call `iced::window::drag_resize` via
`StartMainWindowResize`; anywhere in the middle is an empty
`Space` so events fall through to the content layer below.

Used as a stack layer over `main` rather than as a structural
wrapper, so the content keeps its natural y-origin and overlay
coordinates (Active Bar, text edit, net-colour picker) stay
correct without a +EDGE correction everywhere.

## Source
Lines 453–516 in `crates/oxide-app/src/app/view/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/app/view/chrome.md) |
