---
okf_version: "0.2"
type: Function
title: to_rgba
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/to_rgba
language: rust
---

# to_rgba

## Signature

```rust
fn to_rgba(color: Color) -> [f32; 4]
```

## Source
Lines 368–370 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [draw_power_port_preview](/crates/oxide-app/src/schematic_runtime/mod/draw_power_port_preview.md) |
| called_by | [label_marker_polygon](/crates/oxide-app/src/schematic_runtime/mod/label_marker_polygon.md) |
