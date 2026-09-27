# polygon

## Classs

- [PolygonPipeline](PolygonPipeline.md) — GPU polygon pipeline for filled triangle-list geometry.
- [PolygonVertex](PolygonVertex.md) — [repr(C)]

## Functions

- [append_edge_quad](append_edge_quad.md) — Emit two triangles for the rectangle centred on edge `a -> b`, `half` units
- [append_fan_fill](append_fan_fill.md) — Triangle-fan fill from the first vertex — exact for convex contours only.
- [append_fill](append_fill.md) — Ear-clip fill, concave-safe. Assumes `polygon.vertices.len() >= 3`.
- [append_stroke](append_stroke.md) — Stroke outline: one width-expanded quad per edge of the closed contour
- [concave_contour_fills_exactly_its_own_area](concave_contour_fills_exactly_its_own_area.md) — Correctness — a concave contour (e.g. an L-shaped copper pour) must
- [draw](draw.md)
- [draw](draw_1.md)
- [draw_from](draw_from.md)
- [draw_from](draw_from_1.md)
- [draw_overlay](draw_overlay.md) — Draw all uploaded overlay polygon geometry. Callers composite this in
- [draw_overlay](draw_overlay_1.md) — Draw all uploaded overlay polygon geometry. Callers composite this in
- [emits_a_stroke_outline_after_the_fill](emits_a_stroke_outline_after_the_fill.md) — [test]
- [new](new.md)
- [new](new_1.md)
- [no_stroke_when_color_is_absent](no_stroke_when_color_is_absent.md) — [test]
- [no_stroke_when_width_is_non_positive](no_stroke_when_width_is_non_positive.md) — [test]
- [shoelace_area](shoelace_area.md) — Shoelace area of a closed contour — the ground truth a correct
- [skips_degenerate_polygons](skips_degenerate_polygons.md) — [test]
- [triangle_area](triangle_area.md)
- [triangulate_polygons](triangulate_polygons.md) — Build the triangle-list vertices for a batch of polygons: an ear-clip fill
- [triangulates_every_contour_and_never_treats_it_as_triangle_soup](triangulates_every_contour_and_never_treats_it_as_triangle_soup.md) — [test]
- [triangulates_simple_convex_contour](triangulates_simple_convex_contour.md) — [test]
- [upload](upload.md)
- [upload](upload_1.md)
- [upload_into](upload_into.md)
- [upload_into](upload_into_1.md)
- [upload_overlay](upload_overlay.md) — Upload overlay polygon geometry into the dedicated overlay buffer,
- [upload_overlay](upload_overlay_1.md) — Upload overlay polygon geometry into the dedicated overlay buffer,
- [vertex_count](vertex_count.md)
- [vertex_count](vertex_count_1.md)
