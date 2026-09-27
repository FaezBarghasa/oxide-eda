# order

## Classs

- [SceneBucket](SceneBucket.md) — A drawable bucket of a [`Scene`](crate::scene::Scene). Every variant maps

## Functions

- [circle_fill_predicate_splits_on_stroke_width](circle_fill_predicate_splits_on_stroke_width.md) — [test]
- [cpu_pcb_draw_order_is_main_geometry_then_overlays](cpu_pcb_draw_order_is_main_geometry_then_overlays.md) — [test]
- [cpu_schematic_draw_order_is_base_then_overlays_then_erc](cpu_schematic_draw_order_is_base_then_overlays_then_erc.md) — [test]
- [gpu_and_schematic_cpu_disagree_on_where_fills_go](gpu_and_schematic_cpu_disagree_on_where_fills_go.md) — The divergence this whole module exists to make visible, now stated for
- [gpu_draws_buckets_the_pcb_cpu_path_omits](gpu_draws_buckets_the_pcb_cpu_path_omits.md) — The GPU path composites arc and text buckets the PCB CPU path never
- [gpu_scene_draw_order_is_fills_then_strokes_then_text](gpu_scene_draw_order_is_fills_then_strokes_then_text.md) — [test]
- [index_of](index_of.md)
- [line_dash_predicate_reads_the_low_style_bit](line_dash_predicate_reads_the_low_style_bit.md) — [test]
- [overlays_composite_above_base_buckets](overlays_composite_above_base_buckets.md) — #4 (fixed): both paths composite overlay geometry strictly AFTER every
- [polygon_stroke_predicate_needs_colour_and_width](polygon_stroke_predicate_needs_colour_and_width.md) — [test]
- [scene_shader_composites_no_overlay_or_erc_buckets](scene_shader_composites_no_overlay_or_erc_buckets.md) — Neither draw order composites overlay or ERC buckets through the
- [schematic_overlays_and_erc_markers_composite_above_base_buckets](schematic_overlays_and_erc_markers_composite_above_base_buckets.md) — Overlays and ERC markers are presentation: they must land on top of
- [the_schematic_order_names_every_bucket_exactly_once](the_schematic_order_names_every_bucket_exactly_once.md) — The schematic replay is the only path that draws every bucket, so its
