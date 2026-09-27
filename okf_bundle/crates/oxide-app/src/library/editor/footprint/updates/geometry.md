---
okf_version: "0.2"
type: Module
title: geometry
description: Footprint editor — geometry update logic.
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry
language: rust
---

# geometry

Footprint editor — geometry update logic.

## Docstring

Footprint editor — geometry update logic.

Split out of `apply_footprint_primitive_edit` per ADR-0001 D1/D2.
`apply` is a thin router; each `FootprintEditorMsg` variant delegates
to one named per-action fn below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
| related | [add_new_sibling](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_new_sibling.md) |
| related | [add_pad](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_pad.md) |
| related | [add_via](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_via.md) |
| related | [track_click](/crates/oxide-app/src/library/editor/footprint/updates/geometry/track_click.md) |
| related | [track_cancel](/crates/oxide-app/src/library/editor/footprint/updates/geometry/track_cancel.md) |
| related | [arc_click](/crates/oxide-app/src/library/editor/footprint/updates/geometry/arc_click.md) |
| related | [arc_cancel](/crates/oxide-app/src/library/editor/footprint/updates/geometry/arc_cancel.md) |
| related | [polygon_click](/crates/oxide-app/src/library/editor/footprint/updates/geometry/polygon_click.md) |
| related | [polygon_commit](/crates/oxide-app/src/library/editor/footprint/updates/geometry/polygon_commit.md) |
| related | [polygon_cancel](/crates/oxide-app/src/library/editor/footprint/updates/geometry/polygon_cancel.md) |
| related | [add_text](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_text.md) |
| related | [add_text_frame](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_text_frame.md) |
| related | [add_hole](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_hole.md) |
| related | [mint_body3d](/crates/oxide-app/src/library/editor/footprint/updates/geometry/mint_body3d.md) |
| related | [mint_extruded_body3d](/crates/oxide-app/src/library/editor/footprint/updates/geometry/mint_extruded_body3d.md) |
| related | [footprint_sketch_is_active](/crates/oxide-app/src/library/editor/footprint/updates/geometry/footprint_sketch_is_active.md) |
