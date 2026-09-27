---
okf_version: "0.2"
type: Function
title: ingest_runtime_glb
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb
language: rust
---

# ingest_runtime_glb

## Signature

```rust
pub fn ingest_runtime_glb(
    request: RuntimeGlbIngestRequest,
) -> Result<RuntimeGlbModel, RuntimeGlbIngestError>
```

## Visibility

- `pub`

## Source
Lines 106–119 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [load_glb_bytes](/crates/oxide-renderer/src/pcb3d/glb/load_glb_bytes.md) |
| calls | [validate_and_stage_glb_payload](/crates/oxide-renderer/src/pcb3d/glb/validate_and_stage_glb_payload.md) |
| called_by | [ingest_runtime_model_with_bridge](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_model_with_bridge.md) |
| called_by | [benchmark_smoke_tier_m_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_m_ingest_metadata_counts](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_ingest_metadata_counts.md) |
| called_by | [benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count.md) |
| called_by | [benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count.md) |
| called_by | [benchmark_smoke_tier_s_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_s_ingest_metadata_counts](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_ingest_metadata_counts.md) |
| called_by | [benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count.md) |
| called_by | [benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count.md) |
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
