---
okf_version: "0.2"
type: Module
title: footprint_schema_v3
description: v0.14 footprint schema additions — round-trip tests.
resource: crates/oxide-library/tests/footprint_schema_v3.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/footprint_schema_v3
language: rust
---

# footprint_schema_v3

v0.14 footprint schema additions — round-trip tests.

## Docstring

v0.14 footprint schema additions — round-trip tests.

v0.14 adds optional Vec fields to `Footprint` for closed-profile
bake targets that v0.13 only round-tripped: pours, keepouts,
cutouts, v_scores, mask_openings, mask_excludes, paste_apertures.
It also adds two `PadKind` variants (`Castellated`, `Fiducial`) and
a `PadShape::Chamfered` variant.

All additions are forward + backward compatible:
- new fields use `#[serde(default, skip_serializing_if = "Vec::is_empty")]`
so v2 footprints load with empty Vecs and serialise identically;
- new enum variants are gated behind `#[non_exhaustive]` so callers
already match exhaustively in their own code.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_footprint_uses_v3_schema_version](/crates/oxide-library/tests/footprint_schema_v3/empty_footprint_uses_v3_schema_version.md) |
| related | [v3_pour_round_trips](/crates/oxide-library/tests/footprint_schema_v3/v3_pour_round_trips.md) |
| related | [v3_keepout_round_trips](/crates/oxide-library/tests/footprint_schema_v3/v3_keepout_round_trips.md) |
| related | [v3_cutout_v_score_mask_paste_round_trip](/crates/oxide-library/tests/footprint_schema_v3/v3_cutout_v_score_mask_paste_round_trip.md) |
| related | [text_frame_round_trips_and_defaults_none](/crates/oxide-library/tests/footprint_schema_v3/text_frame_round_trips_and_defaults_none.md) |
| related | [text_without_frame_defaults_to_none_on_legacy_load](/crates/oxide-library/tests/footprint_schema_v3/text_without_frame_defaults_to_none_on_legacy_load.md) |
| related | [v3_castellated_pad_kind_round_trips](/crates/oxide-library/tests/footprint_schema_v3/v3_castellated_pad_kind_round_trips.md) |
| related | [v3_fiducial_pad_kind_round_trips](/crates/oxide-library/tests/footprint_schema_v3/v3_fiducial_pad_kind_round_trips.md) |
| related | [v3_chamfered_pad_shape_round_trips](/crates/oxide-library/tests/footprint_schema_v3/v3_chamfered_pad_shape_round_trips.md) |
| related | [v3_chamfered_corners_all](/crates/oxide-library/tests/footprint_schema_v3/v3_chamfered_corners_all.md) |
| related | [v3_empty_vecs_skip_serialisation](/crates/oxide-library/tests/footprint_schema_v3/v3_empty_vecs_skip_serialisation.md) |
| related | [v2_footprint_loads_without_v3_fields](/crates/oxide-library/tests/footprint_schema_v3/v2_footprint_loads_without_v3_fields.md) |
