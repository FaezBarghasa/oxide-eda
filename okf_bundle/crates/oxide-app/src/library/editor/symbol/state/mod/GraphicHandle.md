---
okf_version: "0.2"
type: Class
title: GraphicHandle
description: "Resize-handle identity for a placed [`SymbolGraphic`]. Each"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/GraphicHandle
language: rust
---

# GraphicHandle

Resize-handle identity for a placed [`SymbolGraphic`]. Each

## Signature

```rust
pub enum GraphicHandle
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Resize-handle identity for a placed [`SymbolGraphic`]. Each
variant identifies one grabbable point on the graphic so the
canvas can fire [`canvas::CanvasAction::MoveGraphicHandle`] with
enough context for the dispatcher to mutate the right field.

Corner ordering for `RectCorner`: `0=TL, 1=TR, 2=BR, 3=BL` in the
Standard y-up world (so TL has minx + maxy).
Edge ordering for `RectEdge`: `0=Top, 1=Right, 2=Bottom, 3=Left`.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 106–131 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
