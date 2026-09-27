---
okf_version: "0.2"
type: Function
title: parse_index
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/parse_index
language: rust
---

# parse_index

## Signature

```rust
fn parse_index(value: &Value, upper_bound: usize, label: String) -> Result<usize, String>
```

## Source
Lines 497–512 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| called_by | [stage_node_tree](/crates/oxide-renderer/src/pcb3d/glb/stage_node_tree.md) |
| called_by | [stage_opaque_primitives](/crates/oxide-renderer/src/pcb3d/glb/stage_opaque_primitives.md) |
