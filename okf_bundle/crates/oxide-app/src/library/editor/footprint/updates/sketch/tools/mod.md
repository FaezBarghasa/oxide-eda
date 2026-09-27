---
okf_version: "0.2"
type: Module
title: tools
description: Footprint sketch updates — tool-click state machine (ADR-0001 D1/D2).
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod
language: rust
---

# tools

Footprint sketch updates — tool-click state machine (ADR-0001 D1/D2).

## Docstring

Footprint sketch updates — tool-click state machine (ADR-0001 D1/D2).

`apply` is a thin router (the one message here, `SketchToolClick`,
delegates to [`handle_tool_click`]). `handle_tool_click` resolves the
click into a [`ToolClickCtx`] (the sketch plane, the snapped-or-minted
click Point, the raw coords, the sticky construction / centerline
flags) through three named steps —
[`resolve_effective_click`] (numeric-placement-input override),
[`resolve_click_point`] (snap/mint the click into an entity id), and
[`try_consume_repick_polar_center`] (the Pattern re-pick intercept) —
then dispatches to the per-tool sub-modules. Bodies moved verbatim.

## Relationships

| Type | Target |
|------|--------|
| related | [ToolClickCtx](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/ToolClickCtx.md) |
| related | [flag](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/flag.md) |
| related | [flag](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/flag.md) |
| related | [ToolDimension](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/ToolDimension.md) |
| related | [resolve_tool_dimension_mm](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_tool_dimension_mm.md) |
| related | [parse_positive_mm](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/parse_positive_mm.md) |
| related | [reject_tool_dimension](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/reject_tool_dimension.md) |
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/apply.md) |
| related | [handle_tool_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click.md) |
| related | [resolve_sketch_plane](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_sketch_plane.md) |
| related | [resolve_effective_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_effective_click.md) |
| related | [resolve_click_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_click_point.md) |
| related | [try_consume_repick_polar_center](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/try_consume_repick_polar_center.md) |
