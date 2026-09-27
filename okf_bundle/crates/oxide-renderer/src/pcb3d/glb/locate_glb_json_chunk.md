---
okf_version: "0.2"
type: Function
title: locate_glb_json_chunk
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/locate_glb_json_chunk
language: rust
---

# locate_glb_json_chunk

## Signature

```rust
fn locate_glb_json_chunk(
    model_id: &str,
    bytes: &'a [u8],
) -> Result<&'a [u8], RuntimeGlbIngestError>
```

## Type Parameters

- `'a`

## Source
Lines 271–307 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [invalid_glb](/crates/oxide-renderer/src/pcb3d/glb/invalid_glb.md) |
| calls | [read_u32_le](/crates/oxide-renderer/src/pcb3d/mod/read_u32_le.md) |
| called_by | [validate_and_stage_glb_payload](/crates/oxide-renderer/src/pcb3d/glb/validate_and_stage_glb_payload.md) |
