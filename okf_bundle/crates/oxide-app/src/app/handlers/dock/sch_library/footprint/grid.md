---
okf_version: "0.2"
type: Module
title: grid
description: Footprint-editor grid / guide / snap-view handlers — the methods
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid
language: rust
---

# grid

Footprint-editor grid / guide / snap-view handlers — the methods

## Docstring

Footprint-editor grid / guide / snap-view handlers — the methods
behind the `FpEditor*` dock-panel messages that manage the Snap
Options sub-tab + snapping mode, the multi-grid Manager (add /
properties / delete / activate), and the guide Manager (add /
delete / toggle / reposition) on the active `.snxfpt` editor. The
dispatcher in `mod.rs` routes these panel messages here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_fp_editor_set_snap_subtab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_set_snap_subtab.md) |
| related | [handle_fp_editor_set_snapping_mode](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_set_snapping_mode.md) |
| related | [handle_fp_editor_grid_manager_add](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_manager_add.md) |
| related | [handle_fp_editor_grid_manager_properties](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_manager_properties.md) |
| related | [handle_fp_editor_grid_manager_delete](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_manager_delete.md) |
| related | [handle_fp_editor_grid_set_active](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_set_active.md) |
| related | [handle_fp_editor_guide_manager_add](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_manager_add.md) |
| related | [handle_fp_editor_guide_add_vertical](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_add_vertical.md) |
| related | [handle_fp_editor_guide_add_horizontal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_add_horizontal.md) |
| related | [handle_fp_editor_guide_delete](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_delete.md) |
| related | [handle_fp_editor_guide_toggle](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_toggle.md) |
| related | [handle_fp_editor_guide_set_position](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_set_position.md) |
| related | [handle_fp_editor_set_snap_subtab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_set_snap_subtab.md) |
| related | [handle_fp_editor_set_snapping_mode](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_set_snapping_mode.md) |
| related | [handle_fp_editor_grid_manager_add](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_manager_add.md) |
| related | [handle_fp_editor_grid_manager_properties](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_manager_properties.md) |
| related | [handle_fp_editor_grid_manager_delete](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_manager_delete.md) |
| related | [handle_fp_editor_grid_set_active](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_set_active.md) |
| related | [handle_fp_editor_guide_manager_add](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_manager_add.md) |
| related | [handle_fp_editor_guide_add_vertical](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_add_vertical.md) |
| related | [handle_fp_editor_guide_add_horizontal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_add_horizontal.md) |
| related | [handle_fp_editor_guide_delete](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_delete.md) |
| related | [handle_fp_editor_guide_toggle](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_toggle.md) |
| related | [handle_fp_editor_guide_set_position](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_set_position.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
