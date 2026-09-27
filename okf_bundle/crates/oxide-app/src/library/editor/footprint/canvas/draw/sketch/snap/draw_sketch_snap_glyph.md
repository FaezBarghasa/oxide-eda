---
okf_version: "0.2"
type: Function
title: draw_sketch_snap_glyph
description: v0.22 Phase A6 — Inferred-constraint snap glyph at the cursor.
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/snap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/snap/draw_sketch_snap_glyph
language: rust
---

# draw_sketch_snap_glyph

v0.22 Phase A6 — Inferred-constraint snap glyph at the cursor.

## Signature

```rust
pub(in crate::library::editor::footprint::canvas::draw) fn draw_sketch_snap_glyph(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    state: &FootprintEditorState,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas::draw)`

## Docstring

v0.22 Phase A6 — Inferred-constraint snap glyph at the cursor.
Rendered AFTER the entity overlay so the badge sits on top of the
underlying geometry. Drives off `cstate.last_snap` which the
cursor-moved handler refreshes via `snap::snap_cursor`. Visible
only while a placement tool is active — Select doesn't draw a
hint because no entity is about to land. Glyphs:
- `●` (filled circle in cyan) — `SnapKind::Point` — auto-Coincident
target; clicking lands a new Point coincident with this one.
- `─` (horizontal cyan bar) — `SnapKind::Horizontal` — auto-H
constraint will land on the new Line.
- `│` (vertical cyan bar) — `SnapKind::Vertical` — auto-V
constraint will land on the new Line.
- `◇` (cyan diamond) — `SnapKind::Angle` — angle-snapped to the
nearest 15° increment.
- Guide / Grid / Raw — silent (Guide already paints its line;
Grid + Raw aren't actionable hints).

## Source
Lines 28–103 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/snap.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| called_by | [draw_sketch_overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_overlays.md) |
