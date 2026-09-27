---
okf_version: "0.2"
type: Module
title: tools
description: "Left-press per-tool gesture arms — the `ButtonPressed(Left)` branch"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/tools
language: rust
---

# tools

Left-press per-tool gesture arms — the `ButtonPressed(Left)` branch

## Docstring

Left-press per-tool gesture arms — the `ButtonPressed(Left)` branch
of the canvas `Program::update` god-function, extracted verbatim as
an `impl SymbolCanvas` method. Select-tool hit-testing (resize
handles, objects, rubber-band start) and every placement tool keep
identical conditions, coordinate math, and `Action` capture/publish
sites.

## Relationships

| Type | Target |
|------|--------|
| related | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
| related | [on_polygon_click](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_polygon_click.md) |
| related | [publish_polygon_commit](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/publish_polygon_commit.md) |
| related | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
| related | [on_polygon_click](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_polygon_click.md) |
| related | [publish_polygon_commit](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/publish_polygon_commit.md) |
| related | [arc_sweep_exceeds_full_turn](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/arc_sweep_exceeds_full_turn.md) |
| related | [arc_sweep_exceeds_full_turn_at_exactly_360](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/arc_sweep_exceeds_full_turn_at_exactly_360.md) |
| related | [arc_sweep_exceeds_full_turn_past_360](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/arc_sweep_exceeds_full_turn_past_360.md) |
| related | [arc_sweep_within_a_turn_is_not_rejected](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/arc_sweep_within_a_turn_is_not_rejected.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
