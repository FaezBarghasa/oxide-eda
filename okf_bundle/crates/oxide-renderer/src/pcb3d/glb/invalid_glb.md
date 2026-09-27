---
okf_version: "0.2"
type: Function
title: invalid_glb
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/invalid_glb
language: rust
---

# invalid_glb

## Signature

```rust
fn invalid_glb(model_id: &str, reason: impl Into<String>) -> RuntimeGlbIngestError
```

## Source
Lines 264–269 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| called_by | [locate_glb_json_chunk](/crates/oxide-renderer/src/pcb3d/glb/locate_glb_json_chunk.md) |
| called_by | [validate_and_stage_glb_payload](/crates/oxide-renderer/src/pcb3d/glb/validate_and_stage_glb_payload.md) |
