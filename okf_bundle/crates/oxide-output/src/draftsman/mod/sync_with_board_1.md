---
okf_version: "0.2"
type: Function
title: sync_with_board
description: Live synchronization from updated PCB board layout.
resource: crates/oxide-output/src/draftsman/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:25:28Z"
concept_id: crates/oxide-output/src/draftsman/mod/sync_with_board_1
language: rust
---

# sync_with_board

Live synchronization from updated PCB board layout.

## Signature

```rust
pub fn sync_with_board(&mut self, board: &PcbBoard)
```

## Visibility

- `pub`

## Docstring

Live synchronization from updated PCB board layout.

## Source
Lines 150–160 in `crates/oxide-output/src/draftsman/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draftsman](/crates/oxide-output/src/draftsman/mod.md) |
