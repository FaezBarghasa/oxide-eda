---
okf_version: "0.2"
type: Function
title: make_glb_with_json
resource: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/make_glb_with_json
language: rust
---

# make_glb_with_json

## Signature

```rust
fn make_glb_with_json(json: &str) -> Vec<u8>
```

## Source
Lines 25–42 in `crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_model_bridge](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.md) |
| called_by | [pcb3d_runtime_bridge_accepts_glb_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_accepts_glb_bytes.md) |
| called_by | [pcb3d_runtime_bridge_passes_through_glb_path_without_conversion](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_passes_through_glb_path_without_conversion.md) |
