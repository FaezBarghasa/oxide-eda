---
okf_version: "0.2"
type: Function
title: include_world_point
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/include_world_point
language: rust
---

# include_world_point

## Signature

```rust
fn include_world_point(bounds: &mut Option<(f32, f32, f32, f32)>, x: f32, y: f32)
```

## Source
Lines 200–209 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| called_by | [include_world_span](/crates/oxide-app/src/pcb_canvas/include_world_span.md) |
| called_by | [renderer_snapshot_bounds](/crates/oxide-app/src/pcb_canvas/renderer_snapshot_bounds.md) |
