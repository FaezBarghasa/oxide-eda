---
okf_version: "0.2"
type: Function
title: pad_from_footprint
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/pad_from_footprint
language: rust
---

# pad_from_footprint

## Signature

```rust
pub(super) fn pad_from_footprint(footprint: &Footprint, pad: &Pad) -> PadInput
```

## Visibility

- `pub(super)`

## Source
Lines 27–44 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [point_to_xy](/crates/oxide-renderer/src/pcb/emit/point_to_xy.md) |
| calls | [rotate_local](/crates/oxide-renderer/src/pcb/emit/rotate_local.md) |
| calls | [mm_with_floor](/crates/oxide-renderer/src/pcb/emit/mm_with_floor.md) |
| called_by | [from_board](/crates/oxide-renderer/src/pcb/mod/from_board.md) |
