---
okf_version: "0.2"
type: Class
title: GraphicSummary
description: "Per-shape Properties-panel summary for a placed `SymbolGraphic`."
resource: crates/oxide-app/src/panels/symbol_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/symbol_context/GraphicSummary
language: rust
---

# GraphicSummary

Per-shape Properties-panel summary for a placed `SymbolGraphic`.

## Signature

```rust
pub struct GraphicSummary
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Per-shape Properties-panel summary for a placed `SymbolGraphic`.
Cloned out of the live `Symbol::graphics` vector each refresh so
the panel doesn't hold a borrow into the editor state.
[derive(Debug, Clone, PartialEq)]

## Methods

- `idx`
- `kind`
- `stroke_width`
- `fill`

## Source
Lines 180–187 in `crates/oxide-app/src/panels/symbol_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_context](/crates/oxide-app/src/panels/symbol_context.md) |
