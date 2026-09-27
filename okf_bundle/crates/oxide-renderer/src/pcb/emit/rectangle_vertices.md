---
okf_version: "0.2"
type: Function
title: rectangle_vertices
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/rectangle_vertices
language: rust
---

# rectangle_vertices

## Signature

```rust
pub(super) fn rectangle_vertices(center: [f32; 2], size_mm: [f32; 2]) -> Vec<[f32; 2]>
```

## Visibility

- `pub(super)`

## Source
Lines 131–141 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [pad_vertices](/crates/oxide-renderer/src/pcb/emit/pad_vertices.md) |
