---
okf_version: "0.2"
type: Class
title: FootprintCanvasState
description: "Canvas-only state owned by `iced::widget::Canvas`. The editor's"
resource: crates/oxide-app/src/library/editor/footprint/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/mod/FootprintCanvasState
language: rust
---

# FootprintCanvasState

Canvas-only state owned by `iced::widget::Canvas`. The editor's

## Signature

```rust
pub struct FootprintCanvasState
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Canvas-only state owned by `iced::widget::Canvas`. The editor's
model lives in `FootprintEditorState`; this struct only holds
per-instance interaction state (camera, drag flags).
[derive(Debug)]

## Methods

- `scale`
- `offset`
- `did_initial_fit`
- `panning`
- `last_pan_pos`
- `pan_moved`
- `box_select_anchor_screen`
- `box_select_current_screen`
- `drag`
- `last_known_selected`
- `last_snap`
- `round_resize_drag`
- `current_modifiers`

## Source
Lines 67–114 in `crates/oxide-app/src/library/editor/footprint/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/footprint/canvas/mod.md) |
