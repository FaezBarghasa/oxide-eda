---
okf_version: "0.2"
type: Class
title: SymbolDisplayOptions
description: Sheet / grid / unit + library identity surfaced on the
resource: crates/oxide-app/src/panels/symbol_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/symbol_context/SymbolDisplayOptions
language: rust
---

# SymbolDisplayOptions

Sheet / grid / unit + library identity surfaced on the

## Signature

```rust
pub struct SymbolDisplayOptions
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Sheet / grid / unit + library identity surfaced on the
Properties panel as Altium "Document Options" when nothing is
selected on the symbol canvas. Mirrors
[`crate::library::state::LibraryDisplaySettings`] but lives in
the panels crate so view code doesn't pull
`crate::library::state` directly.
[derive(Debug, Clone, PartialEq)]

## Methods

- `sheet_color`
- `grid_visible`
- `grid_size_mm`
- `unit`
- `library_name`
- `library_symbol_count`

## Source
Lines 85–97 in `crates/oxide-app/src/panels/symbol_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_context](/crates/oxide-app/src/panels/symbol_context.md) |
