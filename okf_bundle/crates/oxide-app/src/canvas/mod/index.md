# mod

## Classs

- [CanvasEvent](CanvasEvent.md) — [derive(Debug, Clone)]
- [CanvasSlot](CanvasSlot.md) — The canvas program that handles input and rendering.
- [CanvasState](CanvasState.md) — [derive(Debug, Default)]
- [CanvasViewPrefs](CanvasViewPrefs.md) — Everything the schematic `draw` path needs that is **not** per-window
- [ErcMarker](ErcMarker.md) — Canvas-side projection of an ERC violation — just enough to draw
- [ErcMarkerSeverity](ErcMarkerSeverity.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]
- [SchematicCanvas](SchematicCanvas.md) — The schematic `canvas::Program` — a per-frame *view* of the app state,
- [ShapePreviewKind](ShapePreviewKind.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]

## Functions

- [a_consumed_fit_does_not_re_apply](a_consumed_fit_does_not_re_apply.md) — A second consecutive call has nothing to consume. Before #632 the
- [a_pending_fit_reaches_the_camera_in_one_hop](a_pending_fit_reaches_the_camera_in_one_hop.md) — #632 — a fit request used to travel `CanvasSlot::pending_fit` →
- [active_bar_hit](active_bar_hit.md)
- [active_render_cache](active_render_cache.md)
- [active_render_cache](active_render_cache_1.md)
- [active_snapshot](active_snapshot.md)
- [active_snapshot](active_snapshot_1.md)
- [auto_focus_set](auto_focus_set.md) — Compute the "focus" uuid set when auto_focus is on — members of
- [auto_focus_set](auto_focus_set_1.md) — Compute the "focus" uuid set when auto_focus is on — members of
- [camera](camera.md) — Read-only borrow of the camera for the `draw` path. Held for the
- [camera](camera_1.md) — Read-only borrow of the camera for the `draw` path. Held for the
- [camera_mut](camera_mut.md) — Mutable borrow for the `update` path — pan, zoom and fit write here.
- [camera_mut](camera_mut_1.md) — Mutable borrow for the `update` path — pan, zoom and fit write here.
- [clear_bg_cache](clear_bg_cache.md)
- [clear_bg_cache](clear_bg_cache_1.md)
- [clear_content_cache](clear_content_cache.md)
- [clear_content_cache](clear_content_cache_1.md)
- [clear_overlay_cache](clear_overlay_cache.md)
- [clear_overlay_cache](clear_overlay_cache_1.md)
- [default](default.md)
- [default](default_1.md)
- [deref](deref.md)
- [deref](deref_1.md)
- [draw](draw.md)
- [draw](draw_1.md)
- [fit_to_paper](fit_to_paper.md) — Fit the camera to show the schematic content.
- [fit_to_paper](fit_to_paper_1.md) — Fit the camera to show the schematic content.
- [live_camera](live_camera.md) — Current pan/zoom as `(offset_x_px, offset_y_px, scale_px_per_mm)`,
- [live_camera](live_camera_1.md) — Current pan/zoom as `(offset_x_px, offset_y_px, scale_px_per_mm)`,
- [live_camera_reflects_the_single_source_after_a_mutation](live_camera_reflects_the_single_source_after_a_mutation.md) — #632 regression. The camera used to live in `CanvasState` (the
- [mouse_interaction](mouse_interaction.md)
- [mouse_interaction](mouse_interaction_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [set_render_cache](set_render_cache.md)
- [set_render_cache](set_render_cache_1.md)
- [shift_snapshot_for_selection](shift_snapshot_for_selection.md) — Map a relative x position within the Active Bar to a dropdown menu.
- [test_prefs](test_prefs.md) — Minimal render settings for the tests below. `update_pending_fit`
- [update](update.md)
- [update](update_1.md)
