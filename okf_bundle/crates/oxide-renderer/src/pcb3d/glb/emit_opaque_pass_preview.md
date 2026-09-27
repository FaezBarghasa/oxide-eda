---
okf_version: "0.2"
type: Function
title: emit_opaque_pass_preview
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/emit_opaque_pass_preview
language: rust
---

# emit_opaque_pass_preview

## Signature

```rust
pub fn emit_opaque_pass_preview(
    model: &RuntimeGlbModel,
    theme: &ResolvedTheme,
    scene: &mut Scene,
    layout: OpaquePassLayout,
)
```

## Visibility

- `pub`

## Source
Lines 121–160 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [square_vertices](/crates/oxide-renderer/src/pcb3d/mod/square_vertices.md) |
| called_by | [benchmark_smoke_tier_m_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count.md) |
| called_by | [benchmark_smoke_tier_s_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count.md) |
| called_by | [pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons.md) |
| called_by | [pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive.md) |
