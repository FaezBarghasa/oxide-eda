---
okf_version: "0.2"
type: Module
title: types
description: "Canvas surface types — the upward action enum, the rotate-pivot"
resource: crates/oxide-app/src/library/editor/symbol/canvas/types.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/types
language: rust
---

# types

Canvas surface types — the upward action enum, the rotate-pivot

## Docstring

Canvas surface types — the upward action enum, the rotate-pivot
mode, the tool enum, and the Program `State` struct. Pure code
motion out of `mod.rs`; re-exported there (`pub use types::…`) so
external consumers (`standalone.rs`, `documents.rs`) keep importing
them from `…::symbol::canvas`.

## Relationships

| Type | Target |
|------|--------|
| related | [CanvasAction](/crates/oxide-app/src/library/editor/symbol/canvas/types/CanvasAction.md) |
| related | [RotatePivotMode](/crates/oxide-app/src/library/editor/symbol/canvas/types/RotatePivotMode.md) |
| related | [SymbolTool](/crates/oxide-app/src/library/editor/symbol/canvas/types/SymbolTool.md) |
| related | [label](/crates/oxide-app/src/library/editor/symbol/canvas/types/label.md) |
| related | [label](/crates/oxide-app/src/library/editor/symbol/canvas/types/label.md) |
| related | [CanvasState](/crates/oxide-app/src/library/editor/symbol/canvas/types/CanvasState.md) |
