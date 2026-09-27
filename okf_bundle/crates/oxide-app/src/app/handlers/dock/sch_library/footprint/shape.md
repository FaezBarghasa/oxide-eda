---
okf_version: "0.2"
type: Module
title: shape
description: Footprint-editor pour / keepout / cutout / snap / array setters —
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape
language: rust
---

# shape

Footprint-editor pour / keepout / cutout / snap / array setters —

## Docstring

Footprint-editor pour / keepout / cutout / snap / array setters —
the helper methods behind the remaining `FpEditor*` dock-panel
messages that edit copper pours, keepouts, cutouts, the snapping
model, and pad arrays on the active `.snxfpt` editor. The
dispatcher in `mod.rs` routes here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [fp_editor_set_pour_net](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_net.md) |
| related | [fp_editor_set_pour_fill_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_fill_type.md) |
| related | [fp_editor_set_pour_priority](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_priority.md) |
| related | [fp_editor_set_keepout_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_keepout_kind.md) |
| related | [fp_editor_set_cutout_edge_radius](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_edge_radius.md) |
| related | [fp_editor_toggle_snap_option](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_toggle_snap_option.md) |
| related | [handle_fp_set_snap_distance](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/handle_fp_set_snap_distance.md) |
| related | [handle_fp_set_axis_snap_range](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/handle_fp_set_axis_snap_range.md) |
| related | [fp_editor_set_snap_grid_step](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_snap_grid_step.md) |
| related | [fp_editor_set_cutout_through](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_through.md) |
| related | [fp_editor_edit_array_param](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_edit_array_param.md) |
| related | [fp_editor_set_array_numbering_scheme](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_array_numbering_scheme.md) |
| related | [fp_editor_set_bga_skip_letters](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_skip_letters.md) |
| related | [fp_editor_set_bga_start_row](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_row.md) |
| related | [fp_editor_set_bga_start_col](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_col.md) |
| related | [fp_editor_delete_array](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_delete_array.md) |
| related | [fp_editor_begin_repick_polar_center](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_begin_repick_polar_center.md) |
| related | [fp_editor_toggle_array_instance](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_toggle_array_instance.md) |
| related | [fp_editor_set_pour_net](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_net.md) |
| related | [fp_editor_set_pour_fill_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_fill_type.md) |
| related | [fp_editor_set_pour_priority](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_priority.md) |
| related | [fp_editor_set_keepout_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_keepout_kind.md) |
| related | [fp_editor_set_cutout_edge_radius](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_edge_radius.md) |
| related | [fp_editor_toggle_snap_option](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_toggle_snap_option.md) |
| related | [handle_fp_set_snap_distance](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/handle_fp_set_snap_distance.md) |
| related | [handle_fp_set_axis_snap_range](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/handle_fp_set_axis_snap_range.md) |
| related | [fp_editor_set_snap_grid_step](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_snap_grid_step.md) |
| related | [fp_editor_set_cutout_through](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_through.md) |
| related | [fp_editor_edit_array_param](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_edit_array_param.md) |
| related | [fp_editor_set_array_numbering_scheme](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_array_numbering_scheme.md) |
| related | [fp_editor_set_bga_skip_letters](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_skip_letters.md) |
| related | [fp_editor_set_bga_start_row](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_row.md) |
| related | [fp_editor_set_bga_start_col](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_col.md) |
| related | [fp_editor_delete_array](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_delete_array.md) |
| related | [fp_editor_begin_repick_polar_center](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_begin_repick_polar_center.md) |
| related | [fp_editor_toggle_array_instance](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_toggle_array_instance.md) |
