# footprint_pad_sketch_mirror

## Functions

- [app_with_footprint_pads](app_with_footprint_pads.md) — One app with `count` default pads on a footprint editor — the
- [footprint_with_minted_pad](footprint_with_minted_pad.md) — Mint one pad of `shape` at `pos` into a fresh footprint and return
- [footprint_with_two_pads_sharing_a_number](footprint_with_two_pads_sharing_a_number.md) — Two pads that SHARE a pad number, both minted, both synced onto the
- [issue142_delete_does_not_eat_user_geometry_sharing_an_anchor](issue142_delete_does_not_eat_user_geometry_sharing_an_anchor.md) — Deleting a pad must not delete user geometry that merely touches it.
- [issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper](issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper.md) — ...and deleting one of them must leave the other's copper intact.
- [issue142_duplicate_pad_numbers_do_not_alias_one_centre](issue142_duplicate_pad_numbers_do_not_alias_one_centre.md) — Two pads sharing a number must not share a sketch centre.
- [issue142_move_repairs_drifted_bbox_corners](issue142_move_repairs_drifted_bbox_corners.md) — A move re-states the bbox corners absolutely, so drift self-heals.
- [issue142_owned_ledger_survives_a_real_serde_round_trip](issue142_owned_ledger_survives_a_real_serde_round_trip.md) — The durable ledger has to survive the REAL serialiser, not just
- [issue142_post_bake_refresh_does_not_alias_duplicate_numbers](issue142_post_bake_refresh_does_not_alias_duplicate_numbers.md) — The post-bake refresh must not alias duplicate numbers either.
- [issue142_reopened_pad_delete_removes_its_geometry](issue142_reopened_pad_delete_removes_its_geometry.md) — Deleting a reopened pad must remove it, not leave a ghost.
- [issue142_reopened_pad_still_moves_its_whole_outline](issue142_reopened_pad_still_moves_its_whole_outline.md) — A pad reopened from disk must still own its sketch geometry.
- [point_xy](point_xy.md)
- [points](points.md) — Every `Point` in the sketch, as `(id, x, y)`.
- [v026e_paste_does_not_alias_template_shape_params](v026e_paste_does_not_alias_template_shape_params.md) — A pasted pad must not alias the template's sketch parameters.
