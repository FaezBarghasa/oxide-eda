---
okf_version: "0.2"
type: Function
title: stage_opaque_primitives
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/stage_opaque_primitives
language: rust
---

# stage_opaque_primitives

## Signature

```rust
fn stage_opaque_primitives(
    nodes: &[Value],
    scenes: &[Value],
    mesh_layouts: &[MeshLayout],
) -> Result<Vec<RuntimeOpaquePrimitive>, String>
```

## Source
Lines 410–438 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [parse_index](/crates/oxide-renderer/src/pcb3d/glb/parse_index.md) |
| calls | [stage_node_tree](/crates/oxide-renderer/src/pcb3d/glb/stage_node_tree.md) |
| called_by | [validate_and_stage_glb_payload](/crates/oxide-renderer/src/pcb3d/glb/validate_and_stage_glb_payload.md) |
