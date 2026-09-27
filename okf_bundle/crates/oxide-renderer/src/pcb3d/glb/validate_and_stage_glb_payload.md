---
okf_version: "0.2"
type: Function
title: validate_and_stage_glb_payload
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/validate_and_stage_glb_payload
language: rust
---

# validate_and_stage_glb_payload

## Signature

```rust
fn validate_and_stage_glb_payload(
    model_id: &str,
    bytes: &[u8],
) -> Result<(RuntimeGlbMetadata, RuntimeMeshStaging), RuntimeGlbIngestError>
```

## Source
Lines 197–262 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [invalid_glb](/crates/oxide-renderer/src/pcb3d/glb/invalid_glb.md) |
| calls | [read_u32_le](/crates/oxide-renderer/src/pcb3d/mod/read_u32_le.md) |
| calls | [locate_glb_json_chunk](/crates/oxide-renderer/src/pcb3d/glb/locate_glb_json_chunk.md) |
| calls | [parse_json_root](/crates/oxide-renderer/src/pcb3d/glb/parse_json_root.md) |
| calls | [extract_asset_version](/crates/oxide-renderer/src/pcb3d/glb/extract_asset_version.md) |
| calls | [collect_nodes](/crates/oxide-renderer/src/pcb3d/glb/collect_nodes.md) |
| calls | [collect_scenes](/crates/oxide-renderer/src/pcb3d/glb/collect_scenes.md) |
| calls | [collect_mesh_layouts](/crates/oxide-renderer/src/pcb3d/glb/collect_mesh_layouts.md) |
| calls | [stage_opaque_primitives](/crates/oxide-renderer/src/pcb3d/glb/stage_opaque_primitives.md) |
| called_by | [ingest_runtime_glb](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb.md) |
