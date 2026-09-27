---
okf_version: "0.2"
type: Module
title: draw
description: "Footprint sketch tools — drawing tools (carved from `sketch_tools::apply`, ADR-0001 D2)."
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw
language: rust
---

# draw

Footprint sketch tools — drawing tools (carved from `sketch_tools::apply`, ADR-0001 D2).

## Docstring

Footprint sketch tools — drawing tools (carved from `sketch_tools::apply`, ADR-0001 D2).

`apply` is a thin router; each `SketchTool` delegates to one named
per-tool fn below. Bodies moved verbatim; the preamble locals they
read (`flag`, `ctx.plane_id`, `ctx.resolved_id`, raw click
`ctx.x_mm`/`ctx.y_mm`) arrive via [`ToolClickCtx`].

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply.md) |
| related | [select_or_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/select_or_point.md) |
| related | [line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/line.md) |
| related | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| related | [rounded_rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rounded_rectangle.md) |
| related | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| related | [arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/arc.md) |
| related | [edge_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/edge_arc.md) |
| related | [tangent_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/tangent_arc.md) |
