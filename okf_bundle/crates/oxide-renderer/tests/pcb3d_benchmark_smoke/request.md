---
okf_version: "0.2"
type: Function
title: request
description: "---------------------------------------------------------------------------"
resource: crates/oxide-renderer/tests/pcb3d_benchmark_smoke.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_benchmark_smoke/request
language: rust
---

# request

---------------------------------------------------------------------------

## Signature

```rust
fn request(model_id: &str, json: &str) -> RuntimeGlbIngestRequest
```

## Docstring

---------------------------------------------------------------------------
Shared helpers
---------------------------------------------------------------------------

## Source
Lines 24–31 in `crates/oxide-renderer/tests/pcb3d_benchmark_smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_benchmark_smoke](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke.md) |
| calls | [make_glb_bytes](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/make_glb_bytes.md) |
| called_by | [benchmark_smoke_tier_m_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_m_ingest_metadata_counts](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_ingest_metadata_counts.md) |
| called_by | [benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count.md) |
| called_by | [benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count.md) |
| called_by | [benchmark_smoke_tier_s_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_s_ingest_metadata_counts](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_ingest_metadata_counts.md) |
| called_by | [benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count.md) |
| called_by | [benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count.md) |
