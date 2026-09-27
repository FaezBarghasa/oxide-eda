# renderer_scene_canvas

## Classs

- [ArcScreenSpan](ArcScreenSpan.md) — How an [`oxide_gfx::primitive::arc::Arc`] maps onto the screen-space
- [SceneDrawOptions](SceneDrawOptions.md) — [derive(Debug, Clone, Copy)]

## Functions

- [a_degenerate_map_is_not_treated_as_flipped](a_degenerate_map_is_not_treated_as_flipped.md) — A degenerate map must not be read as a reflection — `<` and not `<=`
- [a_full_turn_is_a_circle_regardless_of_handedness](a_full_turn_is_a_circle_regardless_of_handedness.md) — A whole-turn span is a circle on both surfaces — its wrapped sweep
- [a_schematic_arc_bulges_toward_the_point_the_user_clicked](a_schematic_arc_bulges_toward_the_point_the_user_clicked.md) — The bug this fixes, stated as the property that was violated.
- [a_symbol_editor_arc_reflects_because_its_canvas_flips_y](a_symbol_editor_arc_reflects_because_its_canvas_flips_y.md) — The Symbol Editor's map *does* flip Y, so there the negation is
- [a_wrapped_arc_sweeps_the_short_way_in_both_frames](a_wrapped_arc_sweeps_the_short_way_in_both_frames.md) — The wrapped case the sweep convention exists for: 330° → 30° is a 60°
- [arc_screen_span](arc_screen_span.md)
- [arc_screen_span_for](arc_screen_span_for.md) — Map an arc's world angles onto the screen angles lyon will sweep between.
- [both_frames_sweep_the_same_magnitude](both_frames_sweep_the_same_magnitude.md) — Sweep magnitude is a property of the data, not of the surface — only
- [color_from_rgba](color_from_rgba.md)
- [draw_arc_bucket](draw_arc_bucket.md)
- [draw_circle_bucket](draw_circle_bucket.md)
- [draw_dashed_line](draw_dashed_line.md)
- [draw_line_bucket](draw_line_bucket.md)
- [draw_polygon_bucket](draw_polygon_bucket.md)
- [draw_scene_with_world_to_screen](draw_scene_with_world_to_screen.md)
- [draw_text_bucket](draw_text_bucket.md)
- [drawn_mid](drawn_mid.md) — Where the drawn arc's own midpoint lands, as a screen angle: lyon
- [radius_px](radius_px.md)
- [radius_px](radius_px_1.md)
- [span](span.md)
- [stroke_px](stroke_px.md)
- [stroke_px](stroke_px_1.md)
- [text_px](text_px.md)
- [text_px](text_px_1.md)
- [the_schematic_transform_reads_as_y_down](the_schematic_transform_reads_as_y_down.md) — The schematic's real transform, probed. If someone gives
- [the_symbol_editor_transform_reads_as_y_up](the_symbol_editor_transform_reads_as_y_up.md) — The Symbol Editor's real map, in the form its canvas uses.
- [to_text_h_align](to_text_h_align.md)
- [to_text_v_align](to_text_v_align.md)
- [world_is_y_up](world_is_y_up.md) — Does this world→screen map flip Y?
