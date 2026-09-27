---
okf_version: "0.2"
type: Module
title: vscore
description: V-score bake — turns VScoreHintAttr-tagged Line entities into
resource: crates/oxide-bake/src/vscore.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bake/src/vscore
language: rust
---

# vscore

V-score bake — turns VScoreHintAttr-tagged Line entities into

## Docstring

V-score bake — turns VScoreHintAttr-tagged Line entities into
`Footprint::v_scores: Vec<FpVScore>` records.

Phase B / Stage 4 of the v0.14.1 sketch-mode plan. A V-score is a
single straight scoring line on the PCB surface — it's NOT a
closed profile, so the walker is not used. Each tagged Line emits
one record.

The sketch-side `VScoreHintAttr` carries `depth_fraction_expr`
(depth as a fraction of board thickness, evaluated against the
parameter table) and an optional `min_web_expr`. v0.14.1 evaluates
both into mm; the depth_fraction is multiplied by a nominal
[`NOMINAL_BOARD_THICKNESS_MM`] to get an absolute depth (the lib
field is mm, not a fraction). Real fab houses will substitute the
actual board thickness at panelisation time.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_v_scores](/crates/oxide-bake/src/vscore/bake_v_scores.md) |
| related | [map_side](/crates/oxide-bake/src/vscore/map_side.md) |
| related | [opt_eval_mm](/crates/oxide-bake/src/vscore/opt_eval_mm.md) |
| related | [build_ctx](/crates/oxide-bake/src/vscore/build_ctx.md) |
| related | [eval_dimensionless](/crates/oxide-bake/src/vscore/eval_dimensionless.md) |
| related | [solve](/crates/oxide-bake/src/vscore/solve.md) |
| related | [bake_v_score_horizontal_line](/crates/oxide-bake/src/vscore/bake_v_score_horizontal_line.md) |
| related | [bake_v_score_clamps_depth_fraction](/crates/oxide-bake/src/vscore/bake_v_score_clamps_depth_fraction.md) |
| related | [bake_v_score_min_web_baked](/crates/oxide-bake/src/vscore/bake_v_score_min_web_baked.md) |
| related | [bake_v_score_arc_skipped](/crates/oxide-bake/src/vscore/bake_v_score_arc_skipped.md) |
