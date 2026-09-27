---
okf_version: "0.2"
type: Function
title: renderer_snapshot_bounds
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/renderer_snapshot_bounds
language: rust
---

# renderer_snapshot_bounds

## Signature

```rust
fn renderer_snapshot_bounds(snapshot: &PcbSnapshot) -> Option<Rectangle>
```

## Source
Lines 217–276 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| calls | [include_world_span](/crates/oxide-app/src/pcb_canvas/include_world_span.md) |
| calls | [include_world_point](/crates/oxide-app/src/pcb_canvas/include_world_point.md) |
| called_by | [fit_to_board](/crates/oxide-app/src/pcb_canvas/fit_to_board.md) |
