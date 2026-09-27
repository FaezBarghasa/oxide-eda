---
okf_version: "0.2"
type: Function
title: ingest_runtime_model_with_bridge
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_model_with_bridge
language: rust
---

# ingest_runtime_model_with_bridge

## Signature

```rust
pub fn ingest_runtime_model_with_bridge(
    request: RuntimeModelBridgeRequest,
) -> Result<RuntimeModelBridgeResult, RuntimeModelBridgeError>
```

## Visibility

- `pub`

## Source
Lines 6–83 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [ingest_runtime_glb](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb.md) |
| calls | [is_glb_path](/crates/oxide-renderer/src/pcb3d/glb/is_glb_path.md) |
| called_by | [pcb3d_runtime_bridge_accepts_glb_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_accepts_glb_bytes.md) |
| called_by | [pcb3d_runtime_bridge_converts_step_to_glb_and_ingests](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_converts_step_to_glb_and_ingests.md) |
| called_by | [pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests.md) |
| called_by | [pcb3d_runtime_bridge_passes_through_glb_path_without_conversion](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_passes_through_glb_path_without_conversion.md) |
| called_by | [pcb3d_runtime_bridge_reports_import_failure_for_missing_source](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_reports_import_failure_for_missing_source.md) |
