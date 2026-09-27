---
okf_version: "0.2"
type: Function
title: body_rect
description: "Body rectangle, when present, derived from the first"
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/body_rect
language: rust
---

# body_rect

Body rectangle, when present, derived from the first

## Signature

```rust
impl SymbolCanvas<'a> { fn body_rect(&self) -> Option<(f64, f64, f64, f64)> }
```

## Type Parameters

- `'a`

## Docstring

Body rectangle, when present, derived from the first
`SymbolGraphicKind::Rectangle` in `symbol.graphics`.

## Source
Lines 209–216 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
