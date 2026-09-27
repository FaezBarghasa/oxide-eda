# footprint_pad_remint

## Functions

- [a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick](a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick.md) — The other half of (g), and the reason the fix cannot be a plain
- [centre_point](centre_point.md) — Position of the pad's centre `Point`.
- [chamfered_repro_pad](chamfered_repro_pad.md) — The exact repro from issue #390: a Chamfered 2×1 mm pad at the
- [dispatch](dispatch.md)
- [editor_with_minted_pad](editor_with_minted_pad.md) — A footprint editor holding one selected pad whose sketch geometry
- [flip_keeps_the_baked_shape_equal_to_the_editor_shape](flip_keeps_the_baked_shape_equal_to_the_editor_shape.md) — THE INVARIANT (b). `oxide_bake::pad` reads `PadAttr::shape` off the
- [geometry_fingerprint](geometry_fingerprint.md) — Every `Point` position in the sketch, sorted, plus the per-kind
- [horizontal_edge_at_y](horizontal_edge_at_y.md) — The `Line` whose two endpoints both sit at world y == `y` — a pad's
- [one_undo_after_a_rotate_restores_the_prior_sketch_geometry](one_undo_after_a_rotate_restores_the_prior_sketch_geometry.md) — THE INVARIANT (c). The rotate now DROPS and re-mints the sidecar,
- [properties_panel_rotation_regenerates_the_chamfer_anchor](properties_panel_rotation_regenerates_the_chamfer_anchor.md) — THE INVARIANT (d), the Properties-panel rotation field. Structurally
- [resizing_a_chamfered_pad_regenerates_its_chamfer_anchor](resizing_a_chamfered_pad_regenerates_its_chamfer_anchor.md) — THE INVARIANT (e), the size / shape funnel. `with_selected_pad`
- [rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle](rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle.md) — THE INVARIANT (a). One `ActiveBarRotateSelection` on the #390 repro
- [sidecar_point](sidecar_point.md) — Position of the `Point` that a `shape_params` sidecar key names.
- [sketch_centre_drag_carries_the_chamfer_anchor_with_it](sketch_centre_drag_carries_the_chamfer_anchor_with_it.md) — The Sketch-mode CENTRE drag — the one translation path that does not
- [sketch_corner_drag_regenerates_the_chamfer_anchor](sketch_corner_drag_regenerates_the_chamfer_anchor.md) — THE INVARIANT (h), the Sketch-mode corner drag — the edge drag's
- [sketch_edge_drag_regenerates_the_chamfer_anchor](sketch_edge_drag_regenerates_the_chamfer_anchor.md) — THE INVARIANT (g), the Sketch-mode edge drag. Dragging a pad edge
- [translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it](translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it.md) — THE INVARIANT (f), the TRANSLATION siblings — pad drag, nudge,
