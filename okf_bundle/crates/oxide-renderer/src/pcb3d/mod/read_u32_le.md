---
okf_version: "0.2"
type: Function
title: read_u32_le
resource: crates/oxide-renderer/src/pcb3d/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-renderer/src/pcb3d/mod/read_u32_le
language: rust
---

# read_u32_le

## Signature

```rust
pub(super) fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32>
```

## Visibility

- `pub(super)`

## Source
Lines 284–288 in `crates/oxide-renderer/src/pcb3d/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d](/crates/oxide-renderer/src/pcb3d/mod.md) |
| called_by | [locate_glb_json_chunk](/crates/oxide-renderer/src/pcb3d/glb/locate_glb_json_chunk.md) |
| called_by | [validate_and_stage_glb_payload](/crates/oxide-renderer/src/pcb3d/glb/validate_and_stage_glb_payload.md) |
