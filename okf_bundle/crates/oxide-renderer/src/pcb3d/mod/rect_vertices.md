---
okf_version: "0.2"
type: Function
title: rect_vertices
resource: crates/oxide-renderer/src/pcb3d/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-renderer/src/pcb3d/mod/rect_vertices
language: rust
---

# rect_vertices

## Signature

```rust
pub(super) fn rect_vertices(origin: [f32; 2], width: f32, height: f32) -> Vec<[f32; 2]>
```

## Visibility

- `pub(super)`

## Source
Lines 253–262 in `crates/oxide-renderer/src/pcb3d/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d](/crates/oxide-renderer/src/pcb3d/mod.md) |
| called_by | [emit_projection_pass](/crates/oxide-renderer/src/pcb3d/projection/emit_projection_pass.md) |
