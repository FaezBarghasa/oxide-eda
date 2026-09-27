---
okf_version: "0.2"
type: Module
title: sketch
description: Footprint-editor sketch-entity interaction handlers — the methods
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch
language: rust
---

# sketch

Footprint-editor sketch-entity interaction handlers — the methods

## Docstring

Footprint-editor sketch-entity interaction handlers — the methods
behind the `FpEditor*` dock-panel messages that jump between Pads
and Sketch modes, select / hover sketch entities from the
Properties "Conflicts" list, and forward parametric-handle /
parameter-row / corner-radius edits to the sketch dispatcher on
the active `.snxfpt` editor. The dispatcher in `mod.rs` routes
these panel messages here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_fp_editor_edit_pad_in_sketch](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_pad_in_sketch.md) |
| related | [handle_fp_editor_edit_pad_shape_param](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_pad_shape_param.md) |
| related | [handle_fp_editor_unlink_corner_radius](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_unlink_corner_radius.md) |
| related | [handle_fp_editor_edit_sketch_pad_in_pads](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_sketch_pad_in_pads.md) |
| related | [handle_fp_editor_select_sketch_entity](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_select_sketch_entity.md) |
| related | [handle_fp_editor_hover_over_constraint](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_hover_over_constraint.md) |
| related | [handle_fp_editor_edit_parameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_parameter.md) |
| related | [handle_fp_editor_edit_pad_in_sketch](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_pad_in_sketch.md) |
| related | [handle_fp_editor_edit_pad_shape_param](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_pad_shape_param.md) |
| related | [handle_fp_editor_unlink_corner_radius](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_unlink_corner_radius.md) |
| related | [handle_fp_editor_edit_sketch_pad_in_pads](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_sketch_pad_in_pads.md) |
| related | [handle_fp_editor_select_sketch_entity](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_select_sketch_entity.md) |
| related | [handle_fp_editor_hover_over_constraint](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_hover_over_constraint.md) |
| related | [handle_fp_editor_edit_parameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_parameter.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
