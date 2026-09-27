---
okf_version: "0.2"
type: Module
title: active_bar
description: Sketch-mode Active Bar — floating toolbar over the footprint
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar
language: rust
---

# active_bar

Sketch-mode Active Bar — floating toolbar over the footprint

## Docstring

Sketch-mode Active Bar — floating toolbar over the footprint
canvas when the editor is in [`EditorMode::Sketch`].

Mirrors the Fusion 360 sketch toolbar layout, including its
grouping: the geometry tools collapse into two dropdown triggers
(**Create ▾** and **Modify ▾**) while the constraints stay a flat,
selection-driven strip — exactly the split Fusion uses, and for the
same reason. A constraint is applied to a selection you already
made, so burying it costs a click on the most frequent action;
arming a drawing tool happens once and then you draw many, so the
extra click amortises to nothing.

Layout (left → right):

1. **Select** — sketch entity selection tool.
2. **Create ▾** — Line / Rectangle / Rounded Rectangle / Circle /
Arc / Tangent Arc. The trigger borrows the armed tool's icon so
the collapsed bar still shows what's in hand.
3. **Modify ▾** — Fillet / Trim / Mirror / Offset / Rectangular +
Circular Pattern / Make Pad from Profile.
4. **Constrain** — 19 selection-aware constraint buttons. Only the
ones the current selection actually permits are rendered, so
this section is empty until something is selected. Enabled state
derives from the kinds of the primary + secondary selection
slots (+ the extra slot for the two 3-entity Symmetric
constraints).
5. **Dimension input** — `Custom` slot with a `text_input` for
the `DistancePtPt` numeric value.
6. **Linetype** — Normal / Construction / Centerline tri-state pill.

Both dropdown menus live in
[`crate::library::editor::footprint::active_bar_dropdowns`]; this
module only builds their trigger buttons. The bar floats over the
canvas (Stack overlay layer) so it doesn't steal vertical space
from the drawing area.

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/items.md) |
| related | [view](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/view.md) |
| related | [sketch_tool_icon](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/sketch_tool_icon.md) |
| related | [constraint_enable_matrix](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/constraint_enable_matrix.md) |
| related | [tag_index](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/tag_index.md) |
| related | [build_dimension_input](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/build_dimension_input.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
