# mod

## Functions

- [auto_mint_for_literal_pads](auto_mint_for_literal_pads.md) — When the user transitions into Sketch mode for the first time on
- [is_sketch_profile_pad](is_sketch_profile_pad.md) — True when this pad's copper is a traced sketch loop ("Make Pad
- [mint_pad_entities](mint_pad_entities.md) — Mint a pad's centre `Point` + `PadAttr` + per-shape sidecar geometry
- [mint_shape_geometry_for](mint_shape_geometry_for.md) — Branch on `pad.shape` to mint the correct sketch geometry — Circle
- [mirror_add_pad_to_sketch](mirror_add_pad_to_sketch.md) — v0.15 — when a pad is added in Pads mode, mirror the new pad into
- [mirror_delete_pad_from_sketch](mirror_delete_pad_from_sketch.md) — v0.15 — when a pad is deleted in Pads mode, also drop its backing
- [mirror_move_pad_in_sketch](mirror_move_pad_in_sketch.md) — v0.15 — when a pad moves in Pads mode (drag), update its backing
- [point_xy_of](point_xy_of.md) — Raw x/y of a sketch `Point` entity, straight off `SketchData` —
- [profile_seed_line](profile_seed_line.md) — Seed Line of a `Custom(SketchProfile)` pad, read off the `PadAttr`
- [reassert_bbox_corners](reassert_bbox_corners.md) — Re-state the four bbox-corner Points ABSOLUTELY from `pad.bbox_mm()`
- [remint_pad_geometry](remint_pad_geometry.md) — Regenerate a pad's sketch sidecar after a transform that changes
- [rotation_expr](rotation_expr.md) — The sketch-side expression for a pad's rotation. Shared by the
- [sidecar_id](sidecar_id.md) — THE SEEDING RULE, in one place. A `pad.shape_params` VALUE that
- [sketch_is_authored](sketch_is_authored.md) — Whether the footprint's sketch already holds authored (non-
- [translate_profile_with_pad](translate_profile_with_pad.md) — Translate a sketch-profile pad's loop so it tracks the pad to
- [warn_profile_pad_untransformed](warn_profile_pad_untransformed.md) — A sketch-profile pad's copper is a traced loop, not a parametric
