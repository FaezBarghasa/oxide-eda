---
okf_version: "0.2"
type: Function
title: mm_with_floor
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/mm_with_floor
language: rust
---

# mm_with_floor

## Signature

```rust
pub(super) fn mm_with_floor(value: f64, fallback: f64, floor: f64) -> f32
```

## Visibility

- `pub(super)`

## Source
Lines 115–118 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [pad_from_footprint](/crates/oxide-renderer/src/pcb/emit/pad_from_footprint.md) |
| called_by | [trace_from_segment](/crates/oxide-renderer/src/pcb/emit/trace_from_segment.md) |
| called_by | [via_from_board_via](/crates/oxide-renderer/src/pcb/emit/via_from_board_via.md) |
