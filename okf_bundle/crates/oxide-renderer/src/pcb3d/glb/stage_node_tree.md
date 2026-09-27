---
okf_version: "0.2"
type: Function
title: stage_node_tree
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/stage_node_tree
language: rust
---

# stage_node_tree

## Signature

```rust
fn stage_node_tree(
    scene_index: usize,
    root_node: usize,
    nodes: &[Value],
    mesh_layouts: &[MeshLayout],
    staged: &mut Vec<RuntimeOpaquePrimitive>,
) -> Result<(), String>
```

## Source
Lines 440–495 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [parse_index](/crates/oxide-renderer/src/pcb3d/glb/parse_index.md) |
| called_by | [stage_opaque_primitives](/crates/oxide-renderer/src/pcb3d/glb/stage_opaque_primitives.md) |
