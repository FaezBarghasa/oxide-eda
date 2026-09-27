---
okf_version: "0.2"
type: Module
title: pcb3d_runtime_glb_ingest
description: Integration tests for Milestone C runtime GLB ingest hooks.
resource: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest
language: rust
---

# pcb3d_runtime_glb_ingest

Integration tests for Milestone C runtime GLB ingest hooks.

## Docstring

Integration tests for Milestone C runtime GLB ingest hooks.

## Relationships

| Type | Target |
|------|--------|
| related | [request](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/request.md) |
| related | [make_glb_with_json](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/make_glb_with_json.md) |
| related | [valid_minimal_json](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/valid_minimal_json.md) |
| related | [unique_temp_path](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/unique_temp_path.md) |
| related | [pcb3d_runtime_glb_ingest_rejects_non_glb_path_extension](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_non_glb_path_extension.md) |
| related | [pcb3d_runtime_glb_ingest_reports_missing_cache_entry](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_reports_missing_cache_entry.md) |
| related | [pcb3d_runtime_glb_ingest_rejects_invalid_header_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_invalid_header_bytes.md) |
| related | [pcb3d_runtime_glb_ingest_rejects_payload_without_meshes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_payload_without_meshes.md) |
| related | [pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_bytes.md) |
| related | [pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path.md) |
| related | [pcb3d_runtime_glb_ingest_stages_mesh_primitives_for_scene_nodes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_stages_mesh_primitives_for_scene_nodes.md) |
| related | [pcb3d_runtime_glb_ingest_rejects_out_of_range_mesh_index](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_out_of_range_mesh_index.md) |
| related | [pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive.md) |
| related | [pcb3d_projection_alignment_rejects_zero_area_footprint](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_rejects_zero_area_footprint.md) |
| related | [pcb3d_projection_alignment_rejects_uv_out_of_range](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_rejects_uv_out_of_range.md) |
| related | [pcb3d_projection_alignment_rejects_inverted_uv_bounds](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_rejects_inverted_uv_bounds.md) |
| related | [pcb3d_projection_alignment_accepts_valid_config](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_accepts_valid_config.md) |
| related | [pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons.md) |
| related | [pcb3d_projection_pass_emits_one_overlay_per_staged_primitive](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_one_overlay_per_staged_primitive.md) |
| related | [pcb3d_projection_pass_returns_error_on_misaligned_config](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_returns_error_on_misaligned_config.md) |
