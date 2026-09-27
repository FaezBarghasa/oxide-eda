---
okf_version: "0.2"
type: Function
title: benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count
description: "[test]"
resource: crates/oxide-renderer/tests/pcb3d_benchmark_smoke.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_benchmark_smoke/benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count
language: rust
---

# benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count

[test]

## Signature

```rust
fn benchmark_smoke_tier_s_projection_pass_emits_expected_overlay_count()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 98–115 in `crates/oxide-renderer/tests/pcb3d_benchmark_smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_benchmark_smoke](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke.md) |
| calls | [ingest_runtime_glb](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb.md) |
| calls | [request](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/request.md) |
| calls | [tier_s_json](/crates/oxide-renderer/tests/pcb3d_benchmark_smoke/tier_s_json.md) |
| calls | [emit_projection_pass](/crates/oxide-renderer/src/pcb3d/projection/emit_projection_pass.md) |
