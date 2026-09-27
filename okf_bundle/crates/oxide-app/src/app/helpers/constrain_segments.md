---
okf_version: "0.2"
type: Function
title: constrain_segments
description: "Given a start and end point, produce wire segments constrained by the draw mode."
resource: crates/oxide-app/src/app/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/helpers/constrain_segments
language: rust
---

# constrain_segments

Given a start and end point, produce wire segments constrained by the draw mode.

## Signature

```rust
pub(super) fn constrain_segments(
    start: oxide_types::schematic::Point,
    end: oxide_types::schematic::Point,
    mode: DrawMode,
) -> Vec<(oxide_types::schematic::Point, oxide_types::schematic::Point)>
```

## Visibility

- `pub(super)`

## Docstring

Given a start and end point, produce wire segments constrained by the draw mode.
- Ortho90: horizontal then vertical (two segments forming a 90-degree corner)
- Angle45: snap to nearest 45-degree angle (may produce one or two segments)
- FreeAngle: single straight segment

## Source
Lines 78–139 in `crates/oxide-app/src/app/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/app/helpers.md) |
| called_by | [handle_canvas_clicked](/crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked.md) |
