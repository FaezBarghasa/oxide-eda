---
okf_version: "0.2"
type: Class
title: PinRenderGeometry
description: Pre-computed render geometry for one pin.
resource: crates/oxide-app/src/library/editor/symbol/canvas/pins.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/pins/PinRenderGeometry
language: rust
---

# PinRenderGeometry

Pre-computed render geometry for one pin.

## Signature

```rust
pub(super) struct PinRenderGeometry
```

## Visibility

- `pub(super)`

## Docstring

Pre-computed render geometry for one pin.

All positions are in world-mm. Derived once per frame from the pin's
`position`, `orientation`, and `length` so that
`build_symbol_renderer_snapshot` contains only push calls.

## Methods

- `tip`
- `body_end`
- `number_pos`
- `name_pos`
- `text_rotation`
- `name_h_align`

## Source
Lines 109–116 in `crates/oxide-app/src/library/editor/symbol/canvas/pins.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pins](/crates/oxide-app/src/library/editor/symbol/canvas/pins.md) |
