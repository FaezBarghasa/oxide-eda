---
okf_version: "0.2"
type: Function
title: bridge_request
resource: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/bridge_request
language: rust
---

# bridge_request

## Signature

```rust
fn bridge_request(
    model_id: &str,
    source: RuntimeModelSource,
    cache_dir: PathBuf,
) -> RuntimeModelBridgeRequest
```

## Source
Lines 10–23 in `crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_model_bridge](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.md) |
| called_by | [pcb3d_runtime_bridge_accepts_glb_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_accepts_glb_bytes.md) |
| called_by | [pcb3d_runtime_bridge_converts_step_to_glb_and_ingests](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_converts_step_to_glb_and_ingests.md) |
| called_by | [pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests.md) |
| called_by | [pcb3d_runtime_bridge_passes_through_glb_path_without_conversion](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_passes_through_glb_path_without_conversion.md) |
| called_by | [pcb3d_runtime_bridge_reports_import_failure_for_missing_source](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_reports_import_failure_for_missing_source.md) |
