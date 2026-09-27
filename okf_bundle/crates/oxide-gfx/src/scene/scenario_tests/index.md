# scenario_tests

## Functions

- [bucket_count](bucket_count.md)
- [build_full_board_scene](build_full_board_scene.md) — One [`Scene`], every bucket populated: this is the shape a real board with
- [circle_and_polygon_predicates_split_correctly_across_the_scenario](circle_and_polygon_predicates_split_correctly_across_the_scenario.md) — [test]
- [concave_zone](concave_zone.md) — A genuinely non-convex, 6-vertex notched contour standing in for a real
- [concave_zone_fills_exactly_its_area](concave_zone_fills_exactly_its_area.md) — [test]
- [convex_filled_pad](convex_filled_pad.md) — Convex quad standing in for a filled SMD pad: fill only, no stroke.
- [dashed_line](dashed_line.md)
- [filled_and_stroked_polygon](filled_and_stroked_polygon.md) — Filled AND stroked polygon (e.g. a courtyard/silkscreen shape with both a
- [filled_circle](filled_circle.md)
- [full_board_scenario_populates_every_bucket](full_board_scenario_populates_every_bucket.md) — [test]
- [line_style_bit_is_preserved_in_the_scene_ir](line_style_bit_is_preserved_in_the_scene_ir.md) — [test]
- [nonempty_bucket_sequence](nonempty_bucket_sequence.md) — The buckets of `order`, in the order they appear, restricted to the ones
- [outline_circle](outline_circle.md)
- [outline_only_rule_area](outline_only_rule_area.md) — Outline-only rule/keepout area: fully transparent fill (alpha 0), the
- [overlays_composite_above_base_content_in_a_fully_populated_scene](overlays_composite_above_base_content_in_a_fully_populated_scene.md) — #4 (fixed for overlays), tied to real content: `scene::order`'s own
- [shoelace_area](shoelace_area.md) — Shoelace area of a closed contour — the ground truth a correct
- [solid_line](solid_line.md)
- [triangle_area](triangle_area.md)
- [triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles](triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles.md) — [test]
- [triangulate_filled_and_stroked_polygon_appends_stroke_after_fill](triangulate_filled_and_stroked_polygon_appends_stroke_after_fill.md) — [test]
- [triangulate_outline_only_rule_area_still_emits_a_stroke](triangulate_outline_only_rule_area_still_emits_a_stroke.md) — [test]
- [triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum](triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum.md) — [test]
