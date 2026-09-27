---
okf_version: "0.2"
type: Function
title: pcb3d_runtime_bridge_accepts_glb_bytes
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
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_accepts_glb_bytes
language: rust
---

# pcb3d_runtime_bridge_accepts_glb_bytes

[test]

## Signature

```rust
fn pcb3d_runtime_bridge_accepts_glb_bytes()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 112–126 in `crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_model_bridge](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.md) |
| calls | [ingest_runtime_model_with_bridge](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_model_with_bridge.md) |
| calls | [bridge_request](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/bridge_request.md) |
| calls | [make_glb_with_json](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/make_glb_with_json.md) |
| calls | [valid_minimal_json](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/valid_minimal_json.md) |
