---
okf_version: "0.2"
type: Module
title: pcb3d_benchmark_smoke
description: Milestone C benchmark smoke gates for the PCB 3D runtime.
resource: crates/oxide-renderer/tests/pcb3d_benchmark_smoke.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_benchmark_smoke
language: rust
---

# pcb3d_benchmark_smoke

Milestone C benchmark smoke gates for the PCB 3D runtime.

## Docstring

Milestone C benchmark smoke gates for the PCB 3D runtime.

These tests verify that the full pipeline — ingest → mesh staging →
opaque pass → projection pass — produces stable primitive counts for
two canonical fixture tiers (small and medium).

Fixture tiers
─────────────
Tier S  (small)  :  1 scene, 1 node, 1 mesh,  3 primitives
Tier M  (medium) :  2 scenes, 4 nodes, 3 meshes, 7 primitives across nodes

## Relationships

| Type | Target |
|------|--------|
| related | [request](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/request.md) |
| related | [make_glb_bytes](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/make_glb_bytes.md) |
| related | [tier_s_json](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/tier_s_json.md) |
| related | [benchmark_smoke_tier_s_ingest_metadata_counts](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_ingest_metadata_counts.md) |
| related | [benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_opaque_pass_emits_expected_polygon_count.md) |
| related | [benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count.md) |
| related | [benchmark_smoke_tier_s_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_full_pipeline_pass_separation_holds.md) |
| related | [tier_m_json](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/tier_m_json.md) |
| related | [benchmark_smoke_tier_m_ingest_metadata_counts](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_ingest_metadata_counts.md) |
| related | [benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count.md) |
| related | [benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count.md) |
| related | [benchmark_smoke_tier_m_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_full_pipeline_pass_separation_holds.md) |
