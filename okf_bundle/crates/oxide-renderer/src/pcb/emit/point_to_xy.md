---
okf_version: "0.2"
type: Function
title: point_to_xy
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/point_to_xy
language: rust
---

# point_to_xy

## Signature

```rust
pub(super) fn point_to_xy(point: Point) -> [f32; 2]
```

## Visibility

- `pub(super)`

## Source
Lines 111–113 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [pad_from_footprint](/crates/oxide-renderer/src/pcb/emit/pad_from_footprint.md) |
| called_by | [trace_from_segment](/crates/oxide-renderer/src/pcb/emit/trace_from_segment.md) |
| called_by | [via_from_board_via](/crates/oxide-renderer/src/pcb/emit/via_from_board_via.md) |
| called_by | [zone_from_board_zone](/crates/oxide-renderer/src/pcb/emit/zone_from_board_zone.md) |
