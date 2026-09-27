---
okf_version: "0.2"
type: Class
title: SymbolTool
description: "Canvas tools — Altium-style `Tool` enum scoped to this surface."
resource: crates/oxide-app/src/library/editor/symbol/canvas/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/types/SymbolTool
language: rust
---

# SymbolTool

Canvas tools — Altium-style `Tool` enum scoped to this surface.

## Signature

```rust
pub enum SymbolTool
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Canvas tools — Altium-style `Tool` enum scoped to this surface.
Mirrors the SchLib Place menu: Pin / Line / Rectangle / Ellipse
(Circle) / Arc / Text / Polygon are the working tools;
`RoundRectangle` / `Bezier` / `Image` etc. live on the Active Bar
as stubs and are deferred to v0.9.x.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 166–179 in `crates/oxide-app/src/library/editor/symbol/canvas/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-app/src/library/editor/symbol/canvas/types.md) |
