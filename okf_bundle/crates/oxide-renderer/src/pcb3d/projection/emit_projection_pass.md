---
okf_version: "0.2"
type: Function
title: emit_projection_pass
description: "Emit one overlay polygon per staged opaque primitive into `scene.overlay_polygons`."
resource: crates/oxide-renderer/src/pcb3d/projection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/projection/emit_projection_pass
language: rust
---

# emit_projection_pass

Emit one overlay polygon per staged opaque primitive into `scene.overlay_polygons`.

## Signature

```rust
pub fn emit_projection_pass(
    model: &RuntimeGlbModel,
    theme: &ResolvedTheme,
    scene: &mut Scene,
    config: ProjectionPassConfig,
) -> Result<(), ProjectionAlignmentError>
```

## Visibility

- `pub`

## Docstring

Emit one overlay polygon per staged opaque primitive into `scene.overlay_polygons`.

Ordering boundary: this function writes to `scene.overlay_polygons` only.
Callers must call [`emit_opaque_pass_preview`] first (which writes to
`scene.polygons`) to maintain the expected render order: opaque → projection.

Returns `Err` if alignment validation fails; `scene.overlay_polygons` is not
modified in that case.

## Source
Lines 163–210 in `crates/oxide-renderer/src/pcb3d/projection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projection](/crates/oxide-renderer/src/pcb3d/projection.md) |
| calls | [check_projection_alignment](/crates/oxide-renderer/src/pcb3d/projection/check_projection_alignment.md) |
| calls | [rect_vertices](/crates/oxide-renderer/src/pcb3d/mod/rect_vertices.md) |
| called_by | [benchmark_smoke_tier_m_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count.md) |
| called_by | [benchmark_smoke_tier_s_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count.md) |
| called_by | [pcb3d_projection_pass_emits_one_overlay_per_staged_primitive](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_one_overlay_per_staged_primitive.md) |
| called_by | [pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons.md) |
| called_by | [pcb3d_projection_pass_returns_error_on_misaligned_config](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_returns_error_on_misaligned_config.md) |
