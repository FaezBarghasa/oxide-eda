---
okf_version: "0.2"
type: Module
title: transform
description: "Footprint sketch tools — transform & pattern (carved from `sketch_tools::apply`, ADR-0001 D2)."
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform
language: rust
---

# transform

Footprint sketch tools — transform & pattern (carved from `sketch_tools::apply`, ADR-0001 D2).

## Docstring

Footprint sketch tools — transform & pattern (carved from `sketch_tools::apply`, ADR-0001 D2).

`apply` is a thin router; each `SketchTool` delegates to one named
per-tool fn below. Bodies moved verbatim; the preamble locals they
read (`flag`, `ctx.plane_id`, `ctx.resolved_id`, raw click
`ctx.x_mm`/`ctx.y_mm`) arrive via [`ToolClickCtx`].

[`offset`] additionally splits by the source entity's kind
(`offset_line` / `offset_circle` / `offset_arc`) — the three cases are
independent geometric derivations sharing only `source_id`/`dist`/
`ctx`, so splitting them is pure code motion along an existing
conceptual boundary, not a fragmentation of one cohesive routine
(ADR-0001 D2 per-gesture judgment call, #177).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/apply.md) |
| related | [mirror](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/mirror.md) |
| related | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
| related | [offset_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_line.md) |
| related | [offset_circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_circle.md) |
| related | [offset_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_arc.md) |
| related | [rect_pattern](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/rect_pattern.md) |
| related | [circular_pattern](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/circular_pattern.md) |
