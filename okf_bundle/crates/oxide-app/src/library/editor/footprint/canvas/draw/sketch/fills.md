---
okf_version: "0.2"
type: Module
title: fills
description: "Filled closed loops — walks the line/arc graph, finds simple closed"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills
language: rust
---

# fills

Filled closed loops — walks the line/arc graph, finds simple closed

## Docstring

Filled closed loops — walks the line/arc graph, finds simple closed
cycles, and fills each polygon with a role-tinted plate. Also
exposes the `ClosedLoop` records so the click handler can select an
entire loop from a single fill click.

## Relationships

| Type | Target |
|------|--------|
| related | [ClosedLoop](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/ClosedLoop.md) |
| related | [find_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/find_closed_loops.md) |
| related | [pos](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/pos.md) |
| related | [draw_filled_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/draw_filled_closed_loops.md) |
| related | [role_color](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/role_color.md) |
| related | [point_pos](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/point_pos.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
