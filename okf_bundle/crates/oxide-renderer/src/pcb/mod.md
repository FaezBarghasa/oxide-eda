---
okf_version: "0.2"
type: Module
title: pcb
description: PCB 2D scene translator for the first Milestone B vertical slice.
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod
language: rust
---

# pcb

PCB 2D scene translator for the first Milestone B vertical slice.

## Docstring

PCB 2D scene translator for the first Milestone B vertical slice.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: IPC-2612-1, IEEE 315, IEC 60617, wgpu/WGSL public docs.

## Relationships

| Type | Target |
|------|--------|
| related | [TraceInput](/crates/oxide-renderer/src/pcb/mod/TraceInput.md) |
| related | [ViaInput](/crates/oxide-renderer/src/pcb/mod/ViaInput.md) |
| related | [PadInput](/crates/oxide-renderer/src/pcb/mod/PadInput.md) |
| related | [ZonePolygonInput](/crates/oxide-renderer/src/pcb/mod/ZonePolygonInput.md) |
| related | [RatsnestInput](/crates/oxide-renderer/src/pcb/mod/RatsnestInput.md) |
| related | [DrcMarkerInput](/crates/oxide-renderer/src/pcb/mod/DrcMarkerInput.md) |
| related | [PcbSnapshot](/crates/oxide-renderer/src/pcb/mod/PcbSnapshot.md) |
| related | [from_board](/crates/oxide-renderer/src/pcb/mod/from_board.md) |
| related | [with_ratsnest_lines](/crates/oxide-renderer/src/pcb/mod/with_ratsnest_lines.md) |
| related | [with_drc_markers](/crates/oxide-renderer/src/pcb/mod/with_drc_markers.md) |
| related | [from_board](/crates/oxide-renderer/src/pcb/mod/from_board.md) |
| related | [with_ratsnest_lines](/crates/oxide-renderer/src/pcb/mod/with_ratsnest_lines.md) |
| related | [with_drc_markers](/crates/oxide-renderer/src/pcb/mod/with_drc_markers.md) |
| related | [PcbSliceFamily](/crates/oxide-renderer/src/pcb/mod/PcbSliceFamily.md) |
| related | [PcbAppEvent](/crates/oxide-renderer/src/pcb/mod/PcbAppEvent.md) |
| related | [families_for_event](/crates/oxide-renderer/src/pcb/mod/families_for_event.md) |
| related | [dirty_flags_for_families](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_families.md) |
| related | [dirty_flags_for_event](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_event.md) |
| related | [dirty_flags_for_events](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_events.md) |
| related | [PcbRenderer](/crates/oxide-renderer/src/pcb/mod/PcbRenderer.md) |
| related | [build_scene](/crates/oxide-renderer/src/pcb/mod/build_scene.md) |
| related | [build_scene](/crates/oxide-renderer/src/pcb/mod/build_scene.md) |
| related | [sample_board](/crates/oxide-renderer/src/pcb/mod/sample_board.md) |
| related | [pcb_snapshot_collects_trace_via_pad_and_zone_inputs](/crates/oxide-renderer/src/pcb/mod/pcb_snapshot_collects_trace_via_pad_and_zone_inputs.md) |
| related | [pcb_renderer_updates_each_family_with_matching_dirty_flag](/crates/oxide-renderer/src/pcb/mod/pcb_renderer_updates_each_family_with_matching_dirty_flag.md) |
| related | [pcb_overlay_slice_emits_ratsnest_and_drc_primitives](/crates/oxide-renderer/src/pcb/mod/pcb_overlay_slice_emits_ratsnest_and_drc_primitives.md) |
| related | [pcb_drc_violation_type_expands_overlay_line_patterns](/crates/oxide-renderer/src/pcb/mod/pcb_drc_violation_type_expands_overlay_line_patterns.md) |
| related | [pcb_zone_sort_prefers_priority_then_connected_net_for_layer_top](/crates/oxide-renderer/src/pcb/mod/pcb_zone_sort_prefers_priority_then_connected_net_for_layer_top.md) |
| related | [pcb_slice_dirty_mapping_resolves_expected_flags](/crates/oxide-renderer/src/pcb/mod/pcb_slice_dirty_mapping_resolves_expected_flags.md) |
| related | [pcb_event_mapping_routes_to_expected_dirty_flags](/crates/oxide-renderer/src/pcb/mod/pcb_event_mapping_routes_to_expected_dirty_flags.md) |
