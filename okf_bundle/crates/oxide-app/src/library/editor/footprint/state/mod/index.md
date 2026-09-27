# mod

## Classs

- [AlignModal](AlignModal.md) — #370 — "Align…" dialog state. `None` on
- [FootprintEditorState](FootprintEditorState.md) — Live, in-memory state of the Footprint canvas — drives interaction
- [MoveByModal](MoveByModal.md) — v0.14 — typed-delta "Move Selection By X, Y" modal. `None` on

## Functions

- [add_hole_at](add_hole_at.md) — v0.18.12 — click-add a non-plated through hole at the given
- [add_hole_at](add_hole_at_1.md) — v0.18.12 — click-add a non-plated through hole at the given
- [add_pad_assigns_next_number](add_pad_assigns_next_number.md) — [test]
- [add_pad_at](add_pad_at.md) — Click-add a new pad at the given world position.
- [add_pad_at](add_pad_at_1.md) — Click-add a new pad at the given world position.
- [adjust_selection_after_remove](adjust_selection_after_remove.md) — HI-25 helper: when an item is removed at `removed_idx` from a Vec,
- [content_bbox_mm](content_bbox_mm.md) — Bounding box of the entire footprint (pads + courtyard) in mm.
- [content_bbox_mm](content_bbox_mm_1.md) — Bounding box of the entire footprint (pads + courtyard) in mm.
- [delete_pad](delete_pad.md) — Delete the pad at `idx`.
- [delete_pad](delete_pad_1.md) — Delete the pad at `idx`.
- [empty](empty.md) — Empty state — used for brand-new components and as the fallback
- [empty](empty_1.md) — Empty state — used for brand-new components and as the fallback
- [from_footprint](from_footprint.md) — Build canvas state from the primitive's pad list.
- [from_footprint](from_footprint_1.md) — Build canvas state from the primitive's pad list.
- [from_footprint_round_trips_pads](from_footprint_round_trips_pads.md) — [test]
- [move_pad](move_pad.md) — Move the pad at `idx` to a new world position.
- [move_pad](move_pad_1.md) — Move the pad at `idx` to a new world position.
- [next_pad_number](next_pad_number.md) — Auto-incremented pad number — picks the next integer above the
- [next_pad_number](next_pad_number_1.md) — Auto-incremented pad number — picks the next integer above the
- [nudge_pads](nudge_pads.md) — v0.14 — translate every pad in `indices` by `(dx, dy)` mm.
- [nudge_pads](nudge_pads_1.md) — v0.14 — translate every pad in `indices` by `(dx, dy)` mm.
- [pad_at](pad_at.md) — Hit-test pads in reverse z-order (last-drawn = topmost).
- [pad_at](pad_at_1.md) — Hit-test pads in reverse z-order (last-drawn = topmost).
- [parsed](parsed.md) — Parse both buffers as mm. `None` if either fails to parse.
- [parsed](parsed_1.md) — Parse both buffers as mm. `None` if either fails to parse.
- [recompute_courtyard](recompute_courtyard.md) — Recompute the courtyard polygon when auto-fit is enabled.
- [recompute_courtyard](recompute_courtyard_1.md) — Recompute the courtyard polygon when auto-fit is enabled.
- [recompute_courtyard_outline](recompute_courtyard_outline.md) — v0.27 — outline-following courtyard. Builds a polygon for
- [recompute_courtyard_outline](recompute_courtyard_outline_1.md) — v0.27 — outline-following courtyard. Builds a polygon for
- [refresh_pads_from_primitive](refresh_pads_from_primitive.md) — v0.22 Phase D2 — Inverse of `sync_pads_to_primitive`. After a
- [refresh_pads_from_primitive](refresh_pads_from_primitive_1.md) — v0.22 Phase D2 — Inverse of `sync_pads_to_primitive`. After a
- [refresh_relinks_pad_from_sketch_pad_attr](refresh_relinks_pad_from_sketch_pad_attr.md) — A pad that first appears from the SKETCH side — "Make Pad from
- [sync_pads_to_primitive](sync_pads_to_primitive.md) — Write the canvas-side pad list back onto the primitive. Called
- [sync_pads_to_primitive](sync_pads_to_primitive_1.md) — Write the canvas-side pad list back onto the primitive. Called
- [sync_pads_to_primitive_writes_back](sync_pads_to_primitive_writes_back.md) — [test]
- [toggle_auto_fit](toggle_auto_fit.md) — Toggle the auto-fit courtyard flag.
- [toggle_auto_fit](toggle_auto_fit_1.md) — Toggle the auto-fit courtyard flag.
- [with_pads](with_pads.md) — Internal constructor shared by `from_footprint` + `empty`.
- [with_pads](with_pads_1.md) — Internal constructor shared by `from_footprint` + `empty`.
