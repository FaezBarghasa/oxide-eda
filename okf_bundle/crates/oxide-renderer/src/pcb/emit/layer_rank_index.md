---
okf_version: "0.2"
type: Function
title: layer_rank_index
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/layer_rank_index
language: rust
---

# layer_rank_index

## Signature

```rust
pub(super) fn layer_rank_index(board: &PcbBoard) -> HashMap<String, u16>
```

## Visibility

- `pub(super)`

## Source
Lines 74–82 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [from_board](/crates/oxide-renderer/src/pcb/mod/from_board.md) |
