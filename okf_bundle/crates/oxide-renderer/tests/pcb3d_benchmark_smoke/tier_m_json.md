---
okf_version: "0.2"
type: Function
title: tier_m_json
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
concept_id: crates/oxide-renderer/tests/pcb3d_benchmark_smoke/tier_m_json
language: rust
---

# tier_m_json

---------------------------------------------------------------------------

## Signature

```rust
fn tier_m_json() -> &'static str
```

## Docstring

---------------------------------------------------------------------------
Tier M fixture  —  2 scenes, 4 nodes, 3 meshes, 7 primitives across nodes
---------------------------------------------------------------------------

Scene 0: nodes [0, 1]
node 0 → mesh 0 (2 primitives)
node 1 → mesh 1 (3 primitives)
Scene 1: nodes [2, 3]
node 2 → mesh 2 (2 primitives)
node 3 → (no mesh)
Total staged primitives: 2 + 3 + 2 = 7

## Source
Lines 147–166 in `crates/oxide-renderer/tests/pcb3d_benchmark_smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_benchmark_smoke](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke.md) |
| called_by | [benchmark_smoke_tier_m_full_pipeline_pass_separation_holds](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_full_pipeline_pass_separation_holds.md) |
| called_by | [benchmark_smoke_tier_m_ingest_metadata_counts](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_ingest_metadata_counts.md) |
| called_by | [benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_opaque_pass_emits_expected_polygon_count.md) |
| called_by | [benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_m_projection_pass_emits_expected_overlay_count.md) |
