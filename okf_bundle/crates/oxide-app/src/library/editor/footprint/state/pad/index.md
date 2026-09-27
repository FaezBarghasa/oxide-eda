# pad

## Classs

- [AlignOp](AlignOp.md) — v0.14 — active-bar Align / Distribute / Spacing operations. Carried
- [CourtyardRect](CourtyardRect.md) — Auto-fit courtyard rectangle in mm. Built by
- [EditorPad](EditorPad.md) — One pad in the editor canvas. A subset of [`oxide_library::Pad`] —
- [NextPadDefaults](NextPadDefaults.md) — v0.16.3 — author-controlled defaults for the next placed pad.
- [PadSide](PadSide.md) — Pad copper side mirror — UI-side label-bearing enum. The sketch
- [PadStackUi](PadStackUi.md) — v0.20 — UI-side mirror of `Pad`'s pad-stack override fields. All

## Functions

- [bbox_mm](bbox_mm.md) — Un-rotated, axis-aligned half-extent box (min_x, min_y, max_x,
- [bbox_mm](bbox_mm_1.md) — Un-rotated, axis-aligned half-extent box (min_x, min_y, max_x,
- [carry_links_by_unique_number](carry_links_by_unique_number.md) — Carry the three session-volatile link fields from the pre-refresh
- [contains_mm](contains_mm.md) — Point-in-pad containment, rotation-aware. Inverse-rotates the
- [contains_mm](contains_mm_1.md) — Point-in-pad containment, rotation-aware. Inverse-rotates the
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [from](from.md)
- [from](from_1.md)
- [from](from_2.md)
- [from](from_3.md)
- [from_pad](from_pad.md)
- [from_pad](from_pad_1.md)
- [label](label.md)
- [label](label_1.md)
- [local_to_world_mm](local_to_world_mm.md) — Map a POINT given in the pad's own frame — the frame
- [local_to_world_mm](local_to_world_mm_1.md) — Map a POINT given in the pad's own frame — the frame
- [mirror_about_own_vertical_axis](mirror_about_own_vertical_axis.md) — Mirror every mirror-sensitive field about the pad's OWN
- [mirror_about_own_vertical_axis](mirror_about_own_vertical_axis_1.md) — Mirror every mirror-sensitive field about the pad's OWN
- [new_default](new_default.md)
- [new_default](new_default_1.md)
- [new_npt_hole](new_npt_hole.md) — v0.18.12 — non-plated through hole. No copper / mask / paste
- [new_npt_hole](new_npt_hole_1.md) — v0.18.12 — non-plated through hole. No copper / mask / paste
- [pad_pos_key](pad_pos_key.md)
- [pos_key](pos_key.md)
- [primary_layer](primary_layer.md) — Layer the pad lives on for hit-testing / toggle gating.
- [primary_layer](primary_layer_1.md) — Layer the pad lives on for hit-testing / toggle gating.
- [relink_pads_to_sketch](relink_pads_to_sketch.md) — Re-attach each pad's `sketch_entity_id` from the sketch itself, by
- [resolve_link](resolve_link.md) — Pick `pad`'s centre out of the candidates sharing its number, or
- [rotate_delta_to_local_mm](rotate_delta_to_local_mm.md) — Inverse of [`Self::rotate_delta_to_world_mm`].
- [rotate_delta_to_local_mm](rotate_delta_to_local_mm_1.md) — Inverse of [`Self::rotate_delta_to_world_mm`].
- [rotate_delta_to_world_mm](rotate_delta_to_world_mm.md) — Rotate a free VECTOR (a delta — no translation applied) from the
- [rotate_delta_to_world_mm](rotate_delta_to_world_mm_1.md) — Rotate a free VECTOR (a delta — no translation applied) from the
- [rotated_aabb_mm](rotated_aabb_mm.md) — Axis-aligned bounding box of the ROTATED pad, in mm. Equals
- [rotated_aabb_mm](rotated_aabb_mm_1.md) — Axis-aligned bounding box of the ROTATED pad, in mm. Equals
- [rotated_corners_mm](rotated_corners_mm.md) — The four half-extent corners rotated about `position_mm` by
- [rotated_corners_mm](rotated_corners_mm_1.md) — The four half-extent corners rotated about `position_mm` by
- [to_pad](to_pad.md)
- [to_pad](to_pad_1.md)
- [world_to_local_mm](world_to_local_mm.md) — Inverse of [`Self::local_to_world_mm`] — takes a world point into
- [world_to_local_mm](world_to_local_mm_1.md) — Inverse of [`Self::local_to_world_mm`] — takes a world point into
