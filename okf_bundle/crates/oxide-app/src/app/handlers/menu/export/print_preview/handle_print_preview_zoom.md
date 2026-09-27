---
okf_version: "0.2"
type: Function
title: handle_print_preview_zoom
description: "Scroll-wheel zoom on the preview image. `delta_y` follows the"
resource: crates/oxide-app/src/app/handlers/menu/export/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_zoom
language: rust
---

# handle_print_preview_zoom

Scroll-wheel zoom on the preview image. `delta_y` follows the

## Signature

```rust
impl Oxide { pub(crate) fn handle_print_preview_zoom(&mut self, delta_y: f32) }
```

## Visibility

- `pub(crate)`

## Docstring

Scroll-wheel zoom on the preview image. `delta_y` follows the
usual sign convention (positive = scroll up = zoom in). The
step is `ZOOM_STEP` per wheel notch, clamped to
`[ZOOM_MIN, ZOOM_MAX]`. Snapping back below 1× resets the pan
since there's nothing to pan over once the image fits the
viewport.

## Source
Lines 391–408 in `crates/oxide-app/src/app/handlers/menu/export/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview.md) |
