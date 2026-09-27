---
okf_version: "0.2"
type: Function
title: export_board
description: Export all drill files (PTH and NPTH) for a board.
resource: crates/oxide-output/src/drill/excellon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:33:05Z"
concept_id: crates/oxide-output/src/drill/excellon/export_board_1
language: rust
---

# export_board

Export all drill files (PTH and NPTH) for a board.

## Signature

```rust
pub fn export_board(&self, board: &PcbBoard) -> Result<Vec<ExcellonOutput>, DrillError>
```

## Visibility

- `pub`

## Docstring

Export all drill files (PTH and NPTH) for a board.

## Source
Lines 68–137 in `crates/oxide-output/src/drill/excellon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [excellon](/crates/oxide-output/src/drill/excellon.md) |
