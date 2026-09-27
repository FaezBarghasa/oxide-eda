---
okf_version: "0.2"
type: Function
title: from_board
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod/from_board_1
language: rust
---

# from_board

## Signature

```rust
pub fn from_board(board: &PcbBoard) -> Self
```

## Visibility

- `pub`

## Source
Lines 89–126 in `crates/oxide-renderer/src/pcb/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-renderer/src/pcb/mod.md) |
| calls | [layer_rank_index](/crates/oxide-renderer/src/pcb/emit/layer_rank_index.md) |
| calls | [pad_from_footprint](/crates/oxide-renderer/src/pcb/emit/pad_from_footprint.md) |
| calls | [zone_from_board_zone](/crates/oxide-renderer/src/pcb/emit/zone_from_board_zone.md) |
| calls | [sort_zone_stack](/crates/oxide-renderer/src/pcb/emit/sort_zone_stack.md) |
