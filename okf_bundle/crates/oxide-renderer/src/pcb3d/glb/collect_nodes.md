---
okf_version: "0.2"
type: Function
title: collect_nodes
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/collect_nodes
language: rust
---

# collect_nodes

## Signature

```rust
fn collect_nodes(root: &Value) -> Result<&Vec<Value>, String>
```

## Source
Lines 334–345 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| called_by | [validate_and_stage_glb_payload](/crates/oxide-renderer/src/pcb3d/glb/validate_and_stage_glb_payload.md) |
