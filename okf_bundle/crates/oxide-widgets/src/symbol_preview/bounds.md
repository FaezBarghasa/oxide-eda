---
okf_version: "0.2"
type: Function
title: bounds
description: Compute the bounding box of all graphics + pins.
resource: crates/oxide-widgets/src/symbol_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/symbol_preview/bounds
language: rust
---

# bounds

Compute the bounding box of all graphics + pins.

## Signature

```rust
impl SymbolPreview { fn bounds(&self) -> (f64, f64, f64, f64) }
```

## Docstring

Compute the bounding box of all graphics + pins.

## Source
Lines 59–136 in `crates/oxide-widgets/src/symbol_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_preview](/crates/oxide-widgets/src/symbol_preview.md) |
| calls | [pin_stub_direction](/crates/oxide-widgets/src/symbol_preview/pin_stub_direction.md) |
