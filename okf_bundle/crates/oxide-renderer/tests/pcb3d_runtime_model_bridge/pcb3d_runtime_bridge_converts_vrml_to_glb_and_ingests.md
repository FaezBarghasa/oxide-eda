---
okf_version: "0.2"
type: Function
title: pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests
description: "[test]"
resource: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests
language: rust
---

# pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests

[test]

## Signature

```rust
fn pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 129–149 in `crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_model_bridge](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.md) |
| calls | [write_tier0_wrl](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/write_tier0_wrl.md) |
| calls | [ingest_runtime_model_with_bridge](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_model_with_bridge.md) |
| calls | [bridge_request](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/bridge_request.md) |
