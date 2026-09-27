---
okf_version: "0.2"
type: Module
title: entities
description: "Footprint sketch updates — entity placement & drag geometry concern."
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities
language: rust
---

# entities

Footprint sketch updates — entity placement & drag geometry concern.

## Docstring

Footprint sketch updates — entity placement & drag geometry concern.

Carved out of the monolithic `sketch::apply` (ADR-0001 D1/D2). `apply`
is a thin router; each variant delegates to one named per-action fn
below (object→action, ADR-0001 D2).

[`move_line`]'s two-endpoint solver pass (endpoint-slide + arc-tangent
propagation) stays as ONE routine — it is a single cohesive geometric
derivation over shared locals (`id`/`start`/`end`/`dx`/`dy`), and
splitting it further would mean passing that whole entangled state
between helpers for no readability gain (ADR-0001 D2 per-gesture
judgment call, #177). The LATER, mechanically-separate phase — walking
every pad to see whether the dragged line was one of its bbox edges —
genuinely is a distinct concern (pad copper vs. sketch geometry), so
that phase is its own [`propagate_line_drag_to_pad_bboxes`] helper.

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/apply.md) |
| related | [place_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/place_point.md) |
| related | [move_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_point.md) |
| related | [move_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_line.md) |
| related | [propagate_line_drag_to_pad_bboxes](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/propagate_line_drag_to_pad_bboxes.md) |
| related | [resize_round_pad](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/resize_round_pad.md) |
| related | [remint_dragged_pad](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/remint_dragged_pad.md) |
