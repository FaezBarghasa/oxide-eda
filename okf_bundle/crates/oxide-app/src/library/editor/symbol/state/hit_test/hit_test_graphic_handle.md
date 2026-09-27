---
okf_version: "0.2"
type: Function
title: hit_test_graphic_handle
description: "Hit-test world coordinates against every placed graphic's resize"
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_handle
language: rust
---

# hit_test_graphic_handle

Hit-test world coordinates against every placed graphic's resize

## Signature

```rust
pub fn hit_test_graphic_handle(
    sym: &Symbol,
    x: f64,
    y: f64,
    tol_mm: f64,
    active_part: u8,
    selected: &Option<SymbolSelection>,
) -> Option<(usize, GraphicHandle)>
```

## Visibility

- `pub`

## Docstring

Hit-test world coordinates against every placed graphic's resize
handles. Returns `(graphic_idx, handle)` for the first hit, scanning
graphics in reverse so the most-recently-placed graphic wins when
handles overlap.

`tol_mm` is the world-space hit-test radius in millimetres. The
caller should derive it from screen pixels so the hit area stays
consistent at all zoom levels, e.g.:
```text
let tol_mm = (8.0_f32 / camera.scale.max(0.01)).clamp(0.5, 4.0) as f64;
```

`selected` scopes `PolygonVertex` handles to whichever graphic is
currently selected (via [`super::graphic_is_selected`]) — every
OTHER handle kind (rect corner/edge, line endpoint, circle radius,
arc start/end, text anchor) still hit-tests on every graphic on
the part regardless of selection, matching their existing,
unscoped behaviour (and the draw path, which already only ever
*renders* a selected graphic's handles — see
`draw_resize_handles`). A Polygon can carry far more vertices than
any other kind has handles (a Join-into-Polygon result routinely
tessellates an arc side into ~16 points), so scanning them
unconditionally made an unselected polygon effectively
unclickable-as-a-body: a click almost anywhere near it grabbed an
invisible vertex handle instead of falling through to
`hit_test`'s body selection.

## Source
Lines 359–389 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| calls | [graphic_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part.md) |
| calls | [graphic_handles](/crates/oxide-app/src/library/editor/symbol/state/hit_test/graphic_handles.md) |
| calls | [graphic_is_selected](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_is_selected.md) |
| called_by | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
| called_by | [mouse_interaction](/crates/oxide-app/src/library/editor/symbol/canvas/mod/mouse_interaction.md) |
| called_by | [hit_test_graphic_handle_finds_polygon_vertex_when_selected](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_finds_polygon_vertex_when_selected.md) |
| called_by | [hit_test_graphic_handle_finds_rectangle_corner](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_finds_rectangle_corner.md) |
| called_by | [hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected.md) |
