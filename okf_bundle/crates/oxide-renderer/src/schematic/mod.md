---
okf_version: "0.2"
type: Module
title: schematic
description: Schematic renderer interfaces.
resource: crates/oxide-renderer/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/schematic/mod
language: rust
---

# schematic

Schematic renderer interfaces.

## Docstring

Schematic renderer interfaces.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: IPC-2612-1, IEEE 315, IEC 60617, wgpu/WGSL public docs.

## Relationships

| Type | Target |
|------|--------|
| related | [ViewRenderer](/crates/oxide-renderer/src/schematic/mod/ViewRenderer.md) |
| related | [WireInput](/crates/oxide-renderer/src/schematic/mod/WireInput.md) |
| related | [JunctionInput](/crates/oxide-renderer/src/schematic/mod/JunctionInput.md) |
| related | [ArcInput](/crates/oxide-renderer/src/schematic/mod/ArcInput.md) |
| related | [PolygonInput](/crates/oxide-renderer/src/schematic/mod/PolygonInput.md) |
| related | [TextInput](/crates/oxide-renderer/src/schematic/mod/TextInput.md) |
| related | [OverlayLineInput](/crates/oxide-renderer/src/schematic/mod/OverlayLineInput.md) |
| related | [OverlayCircleInput](/crates/oxide-renderer/src/schematic/mod/OverlayCircleInput.md) |
| related | [OverlayPolygonInput](/crates/oxide-renderer/src/schematic/mod/OverlayPolygonInput.md) |
| related | [OverlayInputs](/crates/oxide-renderer/src/schematic/mod/OverlayInputs.md) |
| related | [ErcMarkerInput](/crates/oxide-renderer/src/schematic/mod/ErcMarkerInput.md) |
| related | [SchematicSnapshot](/crates/oxide-renderer/src/schematic/mod/SchematicSnapshot.md) |
| related | [SchematicRenderer](/crates/oxide-renderer/src/schematic/mod/SchematicRenderer.md) |
| related | [build_scene](/crates/oxide-renderer/src/schematic/mod/build_scene.md) |
| related | [build_scene](/crates/oxide-renderer/src/schematic/mod/build_scene.md) |
| related | [make_wire](/crates/oxide-renderer/src/schematic/mod/make_wire.md) |
| related | [make_arc](/crates/oxide-renderer/src/schematic/mod/make_arc.md) |
| related | [make_polygon](/crates/oxide-renderer/src/schematic/mod/make_polygon.md) |
| related | [make_text](/crates/oxide-renderer/src/schematic/mod/make_text.md) |
| related | [make_overlay_line](/crates/oxide-renderer/src/schematic/mod/make_overlay_line.md) |
| related | [make_overlay_polygon](/crates/oxide-renderer/src/schematic/mod/make_overlay_polygon.md) |
| related | [make_overlay_circle](/crates/oxide-renderer/src/schematic/mod/make_overlay_circle.md) |
| related | [default_theme](/crates/oxide-renderer/src/schematic/mod/default_theme.md) |
| related | [build_scene_with_default_theme](/crates/oxide-renderer/src/schematic/mod/build_scene_with_default_theme.md) |
| related | [make_erc_marker](/crates/oxide-renderer/src/schematic/mod/make_erc_marker.md) |
| related | [wire_color_order_prefers_override_then_explicit_then_theme](/crates/oxide-renderer/src/schematic/mod/wire_color_order_prefers_override_then_explicit_then_theme.md) |
| related | [build_scene_updates_requested_primitive_groups_only](/crates/oxide-renderer/src/schematic/mod/build_scene_updates_requested_primitive_groups_only.md) |
| related | [arc_emitter_preserves_wraparound_and_tiny_radius_inputs](/crates/oxide-renderer/src/schematic/mod/arc_emitter_preserves_wraparound_and_tiny_radius_inputs.md) |
| related | [polygon_text_emitters_preserve_mapped_fields](/crates/oxide-renderer/src/schematic/mod/polygon_text_emitters_preserve_mapped_fields.md) |
| related | [text_emitter_covers_all_text_categories](/crates/oxide-renderer/src/schematic/mod/text_emitter_covers_all_text_categories.md) |
| related | [text_emitter_maps_alignment_variants](/crates/oxide-renderer/src/schematic/mod/text_emitter_maps_alignment_variants.md) |
| related | [overlay_emitter_maps_preview_ghost_lasso_snap](/crates/oxide-renderer/src/schematic/mod/overlay_emitter_maps_preview_ghost_lasso_snap.md) |
| related | [erc_marker_emitter_maps_severity_slots_to_palette_colors](/crates/oxide-renderer/src/schematic/mod/erc_marker_emitter_maps_severity_slots_to_palette_colors.md) |
| related | [erc_marker_emitter_handles_dense_marker_cluster](/crates/oxide-renderer/src/schematic/mod/erc_marker_emitter_handles_dense_marker_cluster.md) |
| related | [theme_resolved_flow_updates_wire_and_erc_colors](/crates/oxide-renderer/src/schematic/mod/theme_resolved_flow_updates_wire_and_erc_colors.md) |
