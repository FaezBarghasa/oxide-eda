---
okf_version: "0.2"
type: Module
title: edit
description: "Footprint sketch tools — curve edits (carved from `sketch_tools::apply`, ADR-0001 D2)."
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit
language: rust
---

# edit

Footprint sketch tools — curve edits (carved from `sketch_tools::apply`, ADR-0001 D2).

## Docstring

Footprint sketch tools — curve edits (carved from `sketch_tools::apply`, ADR-0001 D2).

`apply` is a thin router; each `SketchTool` delegates to one named
per-tool fn below. Bodies moved verbatim; the preamble locals they
read (`flag`, `ctx.plane_id`, `ctx.resolved_id`, raw click
`ctx.x_mm`/`ctx.y_mm`) arrive via [`ToolClickCtx`].

[`fillet`]'s two-click gesture splits along its existing state
boundary (`fillet_first_click` picks the first Line;
`fillet_second_click` is the geometric derivation — corner, angle,
tangent points, arc centre — that only runs once both Lines are
known). That is pure code motion: `fillet_second_click` is already
under the ~300-line cap and stays as ONE routine, since it is a
single cohesive derivation feeding straight into entity creation
(ADR-0001 D2 per-gesture judgment call, #177). `trim` and
`break_track` are each already a single cohesive click-driven edit
under the cap and are moved verbatim without further splitting.

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/apply.md) |
| related | [pick_line_at](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at.md) |
| related | [fillet](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet.md) |
| related | [fillet_first_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_first_click.md) |
| related | [fillet_second_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_second_click.md) |
| related | [trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/trim.md) |
| related | [line_xy](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/line_xy.md) |
| related | [pick_line_at_for_trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at_for_trim.md) |
| related | [break_track](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/break_track.md) |
| related | [pick_line_and_param](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_and_param.md) |
