---
okf_version: "0.2"
type: Function
title: symbol_preview
description: Create a symbol preview element.
resource: crates/oxide-widgets/src/symbol_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/symbol_preview/symbol_preview
language: rust
---

# symbol_preview

Create a symbol preview element.

## Signature

```rust
pub fn symbol_preview(symbol: LibSymbol, height: f32) -> Element<'static, ()>
```

## Visibility

- `pub`

## Docstring

Create a symbol preview element.

## Source
Lines 312–317 in `crates/oxide-widgets/src/symbol_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_preview](/crates/oxide-widgets/src/symbol_preview.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
