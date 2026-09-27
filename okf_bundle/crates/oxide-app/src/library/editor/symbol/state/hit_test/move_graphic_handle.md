---
okf_version: "0.2"
type: Function
title: move_graphic_handle
description: "Move the named handle of the graphic at `idx` to world coordinates"
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/move_graphic_handle
language: rust
---

# move_graphic_handle

Move the named handle of the graphic at `idx` to world coordinates

## Signature

```rust
pub fn move_graphic_handle(sym: &mut Symbol, idx: usize, handle: GraphicHandle, x: f64, y: f64)
```

## Visibility

- `pub`

## Docstring

Move the named handle of the graphic at `idx` to world coordinates
`(x, y)`. No-op when `idx` is out of range or the handle variant
doesn't match the graphic kind. For arc endpoints the handle drag
only updates the angle (radius is preserved) so the user can sweep
the arc without resizing it.

## Source
Lines 396–488 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| called_by | [arc_endpoint_handle_drag_survives_save_reload](/crates/oxide-app/src/library/editor/symbol/state/tests/arc_endpoint_handle_drag_survives_save_reload.md) |
| called_by | [move_graphic_handle_moves_line_endpoint](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_moves_line_endpoint.md) |
| called_by | [move_graphic_handle_moves_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_moves_polygon_vertex.md) |
| called_by | [move_graphic_handle_no_op_for_mismatched_variant](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_no_op_for_mismatched_variant.md) |
| called_by | [move_graphic_handle_resizes_circle_radius](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_resizes_circle_radius.md) |
| called_by | [apply_symbol_move](/crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move.md) |
