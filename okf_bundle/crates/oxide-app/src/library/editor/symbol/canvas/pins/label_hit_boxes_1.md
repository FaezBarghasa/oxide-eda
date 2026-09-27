---
okf_version: "0.2"
type: Function
title: label_hit_boxes
description: "Axis-aligned world-mm hit-boxes for the pin's NUMBER and NAME"
resource: crates/oxide-app/src/library/editor/symbol/canvas/pins.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/pins/label_hit_boxes_1
language: rust
---

# label_hit_boxes

Axis-aligned world-mm hit-boxes for the pin's NUMBER and NAME

## Signature

```rust
pub(super) fn label_hit_boxes(&self, pin: &SymbolPin) -> [Aabb; 2]
```

## Visibility

- `pub(super)`

## Docstring

Axis-aligned world-mm hit-boxes for the pin's NUMBER and NAME
labels (in that order), so a pin can be grabbed by its text and
not only by its tip. Reuses `number_pos` / `name_pos` from
`compute` — no offset math is duplicated here.

Pins are only oriented Up/Down/Left/Right, so the text runs
horizontally (Left/Right pins) or vertically (Up/Down pins). The
box is centred on the label anchor and grown by half its extent
on each axis, swapping width/height for vertical text. An empty
label yields a degenerate box that never hits.

## Source
Lines 208–228 in `crates/oxide-app/src/library/editor/symbol/canvas/pins.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pins](/crates/oxide-app/src/library/editor/symbol/canvas/pins.md) |
