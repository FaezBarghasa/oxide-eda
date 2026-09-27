---
okf_version: "0.2"
type: Function
title: check_projection_alignment
description: "Validate that `config` is geometrically consistent for projection."
resource: crates/oxide-renderer/src/pcb3d/projection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/projection/check_projection_alignment
language: rust
---

# check_projection_alignment

Validate that `config` is geometrically consistent for projection.

## Signature

```rust
pub fn check_projection_alignment(
    model_id: &str,
    config: &ProjectionPassConfig,
) -> Result<(), ProjectionAlignmentError>
```

## Visibility

- `pub`

## Docstring

Validate that `config` is geometrically consistent for projection.

Returns `Ok(())` when:
- The footprint bounds have positive area on both axes.
- All UV bound values are in `[0.0, 1.0]`.
- UV min is strictly less than UV max on both axes.

## Source
Lines 112–153 in `crates/oxide-renderer/src/pcb3d/projection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projection](/crates/oxide-renderer/src/pcb3d/projection.md) |
| called_by | [emit_projection_pass](/crates/oxide-renderer/src/pcb3d/projection/emit_projection_pass.md) |
| called_by | [pcb3d_projection_alignment_accepts_valid_config](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_accepts_valid_config.md) |
| called_by | [pcb3d_projection_alignment_rejects_inverted_uv_bounds](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_rejects_inverted_uv_bounds.md) |
| called_by | [pcb3d_projection_alignment_rejects_uv_out_of_range](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_rejects_uv_out_of_range.md) |
| called_by | [pcb3d_projection_alignment_rejects_zero_area_footprint](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_alignment_rejects_zero_area_footprint.md) |
