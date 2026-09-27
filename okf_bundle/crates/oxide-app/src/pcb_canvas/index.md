# pcb_canvas

## Classs

- [PcbCanvas](PcbCanvas.md)
- [PcbCanvasState](PcbCanvasState.md) — [derive(Debug, Default)]

## Functions

- [active_renderer_snapshot](active_renderer_snapshot.md)
- [active_renderer_snapshot](active_renderer_snapshot_1.md)
- [build_scene](build_scene.md) — Build the `oxide_gfx` scene for a board snapshot. Shared by the CPU
- [build_scene](build_scene_1.md) — Build the `oxide_gfx` scene for a board snapshot. Shared by the CPU
- [clear_bg_cache](clear_bg_cache.md)
- [clear_bg_cache](clear_bg_cache_1.md)
- [clear_content_cache](clear_content_cache.md)
- [clear_content_cache](clear_content_cache_1.md)
- [clear_content_cache_invalidates_the_gpu_scene](clear_content_cache_invalidates_the_gpu_scene.md) — [test]
- [color_from_rgba](color_from_rgba.md)
- [default](default.md)
- [default](default_1.md)
- [draw](draw.md)
- [draw](draw_1.md)
- [draw_circles](draw_circles.md)
- [draw_dashed_line](draw_dashed_line.md)
- [draw_lines](draw_lines.md)
- [draw_polygons](draw_polygons.md)
- [draw_scene](draw_scene.md)
- [fit_to_board](fit_to_board.md)
- [fit_to_board](fit_to_board_1.md)
- [gpu_scene](gpu_scene.md) — Build the scene for GPU rendering: the same geometry as the CPU path.
- [gpu_scene](gpu_scene_1.md) — Build the scene for GPU rendering: the same geometry as the CPU path.
- [gpu_scene_builds_once_then_serves_from_cache](gpu_scene_builds_once_then_serves_from_cache.md) — [test]
- [gpu_scene_is_none_without_a_snapshot](gpu_scene_is_none_without_a_snapshot.md) — [test]
- [gpu_scene_keeps_a_stable_generation_across_cache_hits](gpu_scene_keeps_a_stable_generation_across_cache_hits.md) — [test]
- [gpu_scene_keeps_an_overlay_polygon_out_of_the_main_bucket](gpu_scene_keeps_an_overlay_polygon_out_of_the_main_bucket.md) — [test]
- [gpu_scene_keeps_overlays_out_of_the_base_buckets](gpu_scene_keeps_overlays_out_of_the_base_buckets.md) — Correctness — thread #4 (z-order): `gpu_scene()` must NOT fold overlay
- [include_world_point](include_world_point.md)
- [include_world_span](include_world_span.md)
- [live_camera](live_camera.md) — Current pan/zoom for the GPU path as
- [live_camera](live_camera_1.md) — Current pan/zoom for the GPU path as
- [live_camera_reflects_the_single_source_after_a_mutation](live_camera_reflects_the_single_source_after_a_mutation.md) — [test]
- [mouse_interaction](mouse_interaction.md)
- [mouse_interaction](mouse_interaction_1.md)
- [new](new.md)
- [new](new_1.md)
- [renderer_snapshot_bounds](renderer_snapshot_bounds.md)
- [scene_generation](scene_generation.md) — Generation id of the scene [`Self::gpu_scene`] currently returns. Passed
- [scene_generation](scene_generation_1.md) — Generation id of the scene [`Self::gpu_scene`] currently returns. Passed
- [scene_generation_bumps_on_every_invalidation](scene_generation_bumps_on_every_invalidation.md) — [test]
- [set_renderer_snapshot](set_renderer_snapshot.md)
- [set_renderer_snapshot](set_renderer_snapshot_1.md)
- [set_theme_colors](set_theme_colors.md)
- [set_theme_colors](set_theme_colors_1.md)
- [setting_a_new_snapshot_invalidates_the_gpu_scene](setting_a_new_snapshot_invalidates_the_gpu_scene.md) — [test]
- [update](update.md)
- [update](update_1.md)
- [world_to_screen](world_to_screen.md)
