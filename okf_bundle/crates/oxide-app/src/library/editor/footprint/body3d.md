---
okf_version: "0.2"
type: Module
title: body3d
description: Body 3D editor pane.
resource: crates/oxide-app/src/library/editor/footprint/body3d.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d
language: rust
---

# body3d

Body 3D editor pane.

## Docstring

Body 3D editor pane.

Sits in the right column of the Footprint tab (per
`v0.9-refactor-2-plan.md` §11 step F3). Edits the
[`oxide_library::Body3D`] embedded on the active footprint
primitive — the procedural 3D render in `preview3d.rs` rebuilds off
these values on every frame.

## Relationships

| Type | Target |
|------|--------|
| related | [ShapePick](/crates/oxide-app/src/library/editor/footprint/body3d/ShapePick.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/footprint/body3d/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/footprint/body3d/fmt.md) |
| related | [view](/crates/oxide-app/src/library/editor/footprint/body3d/view.md) |
| related | [labeled_field](/crates/oxide-app/src/library/editor/footprint/body3d/labeled_field.md) |
| related | [color_row](/crates/oxide-app/src/library/editor/footprint/body3d/color_row.md) |
| related | [body3d_default_is_a_sensible_block](/crates/oxide-app/src/library/editor/footprint/body3d/body3d_default_is_a_sensible_block.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
| related | [iced_aw](/_dependencies/cargo/iced_aw.md) |
