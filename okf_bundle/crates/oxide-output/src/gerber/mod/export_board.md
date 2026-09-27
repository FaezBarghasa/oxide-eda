---
okf_version: "0.2"
type: Function
title: export_board
description: Export all production layers for a given PCB board.
resource: crates/oxide-output/src/gerber/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:02:33Z"
concept_id: crates/oxide-output/src/gerber/mod/export_board
language: rust
---

# export_board

Export all production layers for a given PCB board.

## Signature

```rust
impl GerberExporter { pub fn export_board(&self, board: &PcbBoard) -> Result<Vec<GerberLayerOutput>, GerberError> }
```

## Visibility

- `pub`

## Docstring

Export all production layers for a given PCB board.

## Source
Lines 120–152 in `crates/oxide-output/src/gerber/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gerber](/crates/oxide-output/src/gerber/mod.md) |
