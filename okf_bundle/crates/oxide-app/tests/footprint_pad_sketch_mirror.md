---
okf_version: "0.2"
type: Module
title: footprint_pad_sketch_mirror
description: "Pad ↔ sketch mirror regressions (issue #142 remediation)."
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror
language: rust
---

# footprint_pad_sketch_mirror

Pad ↔ sketch mirror regressions (issue #142 remediation).

## Docstring

Pad ↔ sketch mirror regressions (issue #142 remediation).

Split out of `regression.rs` rather than appended to it — that file
is past 6500 lines and the repo caps a file at ~800.

The wave-1 fix for #142 routed move / delete / paste through one
owned-entity set, but proved it only INSIDE the minting session.
Every assertion here is about what survives the boundary the
session-local tests could not see — a save + reopen — plus the two
blast-radius properties the widened owned set put at risk.

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_with_minted_pad](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_minted_pad.md) |
| related | [footprint_with_two_pads_sharing_a_number](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_two_pads_sharing_a_number.md) |
| related | [points](/crates/oxide-app/tests/footprint_pad_sketch_mirror/points.md) |
| related | [point_xy](/crates/oxide-app/tests/footprint_pad_sketch_mirror/point_xy.md) |
| related | [issue142_reopened_pad_still_moves_its_whole_outline](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_still_moves_its_whole_outline.md) |
| related | [issue142_reopened_pad_delete_removes_its_geometry](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_delete_removes_its_geometry.md) |
| related | [issue142_delete_does_not_eat_user_geometry_sharing_an_anchor](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_delete_does_not_eat_user_geometry_sharing_an_anchor.md) |
| related | [issue142_move_repairs_drifted_bbox_corners](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_move_repairs_drifted_bbox_corners.md) |
| related | [issue142_duplicate_pad_numbers_do_not_alias_one_centre](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_duplicate_pad_numbers_do_not_alias_one_centre.md) |
| related | [issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper.md) |
| related | [issue142_owned_ledger_survives_a_real_serde_round_trip](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_owned_ledger_survives_a_real_serde_round_trip.md) |
| related | [app_with_footprint_pads](/crates/oxide-app/tests/footprint_pad_sketch_mirror/app_with_footprint_pads.md) |
| related | [v026e_paste_does_not_alias_template_shape_params](/crates/oxide-app/tests/footprint_pad_sketch_mirror/v026e_paste_does_not_alias_template_shape_params.md) |
| related | [issue142_post_bake_refresh_does_not_alias_duplicate_numbers](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_post_bake_refresh_does_not_alias_duplicate_numbers.md) |
