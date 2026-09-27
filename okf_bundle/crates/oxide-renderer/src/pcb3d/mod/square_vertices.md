---
okf_version: "0.2"
type: Function
title: square_vertices
description: "---------------------------------------------------------------------------"
resource: crates/oxide-renderer/src/pcb3d/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-renderer/src/pcb3d/mod/square_vertices
language: rust
---

# square_vertices

---------------------------------------------------------------------------

## Signature

```rust
pub(super) fn square_vertices(origin: [f32; 2], size: f32) -> Vec<[f32; 2]>
```

## Visibility

- `pub(super)`

## Docstring

---------------------------------------------------------------------------
Geometry helpers
---------------------------------------------------------------------------

## Source
Lines 268–277 in `crates/oxide-renderer/src/pcb3d/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d](/crates/oxide-renderer/src/pcb3d/mod.md) |
| called_by | [emit_opaque_pass_preview](/crates/oxide-renderer/src/pcb3d/glb/emit_opaque_pass_preview.md) |
