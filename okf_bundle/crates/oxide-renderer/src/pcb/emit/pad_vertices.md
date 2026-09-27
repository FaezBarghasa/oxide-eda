---
okf_version: "0.2"
type: Function
title: pad_vertices
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/pad_vertices
language: rust
---

# pad_vertices

## Signature

```rust
pub(super) fn pad_vertices(pad: &PadInput) -> Vec<[f32; 2]>
```

## Visibility

- `pub(super)`

## Source
Lines 162–177 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [ellipse_vertices](/crates/oxide-renderer/src/pcb/emit/ellipse_vertices.md) |
| calls | [rectangle_vertices](/crates/oxide-renderer/src/pcb/emit/rectangle_vertices.md) |
| called_by | [emit_static_polygons](/crates/oxide-renderer/src/pcb/emit/emit_static_polygons.md) |
