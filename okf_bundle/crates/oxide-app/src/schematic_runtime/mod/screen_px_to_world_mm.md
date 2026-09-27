---
okf_version: "0.2"
type: Function
title: screen_px_to_world_mm
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/screen_px_to_world_mm
language: rust
---

# screen_px_to_world_mm

## Signature

```rust
fn screen_px_to_world_mm(px: f32, scale: f32) -> f64
```

## Source
Lines 377–379 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [draw_power_port_preview](/crates/oxide-app/src/schematic_runtime/mod/draw_power_port_preview.md) |
