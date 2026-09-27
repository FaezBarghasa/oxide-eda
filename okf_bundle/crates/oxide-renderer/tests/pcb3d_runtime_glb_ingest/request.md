---
okf_version: "0.2"
type: Function
title: request
resource: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/request
language: rust
---

# request

## Signature

```rust
fn request(model_id: &str, glb_source: GlbSource) -> RuntimeGlbIngestRequest
```

## Source
Lines 14–21 in `crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_glb_ingest](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.md) |
| called_by | [pcb3d_projection_pass_emits_one_overlay_per_staged_primitive](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_one_overlay_per_staged_primitive.md) |
| called_by | [pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons.md) |
| called_by | [pcb3d_projection_pass_returns_error_on_misaligned_config](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_returns_error_on_misaligned_config.md) |
| called_by | [pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_bytes.md) |
| called_by | [pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path.md) |
| called_by | [pcb3d_runtime_glb_ingest_rejects_invalid_header_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_invalid_header_bytes.md) |
| called_by | [pcb3d_runtime_glb_ingest_rejects_non_glb_path_extension](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_non_glb_path_extension.md) |
| called_by | [pcb3d_runtime_glb_ingest_rejects_out_of_range_mesh_index](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_out_of_range_mesh_index.md) |
| called_by | [pcb3d_runtime_glb_ingest_rejects_payload_without_meshes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_rejects_payload_without_meshes.md) |
| called_by | [pcb3d_runtime_glb_ingest_reports_missing_cache_entry](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_reports_missing_cache_entry.md) |
| called_by | [pcb3d_runtime_glb_ingest_stages_mesh_primitives_for_scene_nodes](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_stages_mesh_primitives_for_scene_nodes.md) |
| called_by | [pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive.md) |
