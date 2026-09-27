---
okf_version: "0.2"
type: Function
title: include_world_span
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/include_world_span
language: rust
---

# include_world_span

## Signature

```rust
fn include_world_span(bounds: &mut Option<(f32, f32, f32, f32)>, x: f32, y: f32, radius: f32)
```

## Source
Lines 211–215 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| calls | [include_world_point](/crates/oxide-app/src/pcb_canvas/include_world_point.md) |
| called_by | [renderer_snapshot_bounds](/crates/oxide-app/src/pcb_canvas/renderer_snapshot_bounds.md) |
