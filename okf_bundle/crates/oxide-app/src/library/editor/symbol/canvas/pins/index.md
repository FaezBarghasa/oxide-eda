# pins

## Classs

- [Aabb](Aabb.md) — A world-mm axis-aligned bounding box used for label hit-testing.
- [PinRenderGeometry](PinRenderGeometry.md) — Pre-computed render geometry for one pin.
- [PinTextLayout](PinTextLayout.md) — [derive(Debug, Clone, Copy)]
- [SymbolPalette](SymbolPalette.md) — Palette derived from the active sheet colour — picks a content

## Functions

- [compute](compute.md)
- [compute](compute_1.md)
- [contains](contains.md) — True when `(x, y)` lies inside (or on the edge of) the box.
- [contains](contains_1.md) — True when `(x, y)` lies inside (or on the edge of) the box.
- [empty](empty.md) — A degenerate, never-containing box — the guard for empty labels.
- [empty](empty_1.md) — A degenerate, never-containing box — the guard for empty labels.
- [empty_label_box_never_hits](empty_label_box_never_hits.md) — [test]
- [for_sheet](for_sheet.md)
- [for_sheet](for_sheet_1.md)
- [label_boxes_grab_their_anchor_and_reject_far_points](label_boxes_grab_their_anchor_and_reject_far_points.md) — [test]
- [label_hit_boxes](label_hit_boxes.md) — Axis-aligned world-mm hit-boxes for the pin's NUMBER and NAME
- [label_hit_boxes](label_hit_boxes_1.md) — Axis-aligned world-mm hit-boxes for the pin's NUMBER and NAME
- [text_box](text_box.md) — One label's hit-box: estimate the rendered width from the glyph
- [text_box](text_box_1.md) — One label's hit-box: estimate the rendered width from the glyph
