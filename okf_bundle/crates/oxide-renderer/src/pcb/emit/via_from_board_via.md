---
okf_version: "0.2"
type: Function
title: via_from_board_via
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/via_from_board_via
language: rust
---

# via_from_board_via

## Signature

```rust
pub(super) fn via_from_board_via(via: &Via) -> ViaInput
```

## Visibility

- `pub(super)`

## Source
Lines 14–25 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [point_to_xy](/crates/oxide-renderer/src/pcb/emit/point_to_xy.md) |
| calls | [mm_with_floor](/crates/oxide-renderer/src/pcb/emit/mm_with_floor.md) |
