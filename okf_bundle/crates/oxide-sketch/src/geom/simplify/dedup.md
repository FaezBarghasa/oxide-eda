---
okf_version: "0.2"
type: Function
title: dedup
description: "Drop adjacent duplicate vertices (within `eps`). The closing"
resource: crates/oxide-sketch/src/geom/simplify.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/simplify/dedup
language: rust
---

# dedup

Drop adjacent duplicate vertices (within `eps`). The closing

## Signature

```rust
pub fn dedup(polygon: &[Point2], eps: f64) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Drop adjacent duplicate vertices (within `eps`). The closing
vertex of a closed ring isn't stored in the input convention
(last != first), so the wrap-around comparison runs separately.

## Source
Lines 42–61 in `crates/oxide-sketch/src/geom/simplify.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simplify](/crates/oxide-sketch/src/geom/simplify.md) |
| called_by | [netlist_references](/crates/oxide-app/src/app/handlers/menu/export/tests/netlist_references.md) |
| called_by | [view_pdf_structure_section](/crates/oxide-app/src/app/view/pdf_preview/settings/view_pdf_structure_section.md) |
| called_by | [mirror_move_roundrect_translates_anchors_and_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres.md) |
| called_by | [selected_pad_indices](/crates/oxide-app/src/library/editor/footprint/state/selection/selected_pad_indices.md) |
| called_by | [align_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_confirm.md) |
| called_by | [delete_selected](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_selected.md) |
| called_by | [select_pads](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_pads.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
| called_by | [cull_items](/crates/oxide-gfx/src/scene/upload/mod/cull_items.md) |
| called_by | [convex_hull](/crates/oxide-sketch/src/geom/hull/convex_hull.md) |
| called_by | [dedup_removes_adjacent_duplicates](/crates/oxide-sketch/src/geom/simplify/dedup_removes_adjacent_duplicates.md) |
| called_by | [dedup_removes_wrap_around_duplicate](/crates/oxide-sketch/src/geom/simplify/dedup_removes_wrap_around_duplicate.md) |
| called_by | [simplify_polygon](/crates/oxide-sketch/src/geom/simplify/simplify_polygon.md) |
