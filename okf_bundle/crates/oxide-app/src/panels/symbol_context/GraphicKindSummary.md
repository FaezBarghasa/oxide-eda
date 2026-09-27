---
okf_version: "0.2"
type: Class
title: GraphicKindSummary
description: "Per-variant geometry for [`GraphicSummary`] — mirrors"
resource: crates/oxide-app/src/panels/symbol_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/symbol_context/GraphicKindSummary
language: rust
---

# GraphicKindSummary

Per-variant geometry for [`GraphicSummary`] — mirrors

## Signature

```rust
pub enum GraphicKindSummary
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Per-variant geometry for [`GraphicSummary`] — mirrors
`oxide_library::SymbolGraphicKind` so the panel can render each
shape's editable fields without depending on the library type.
[derive(Debug, Clone, PartialEq)]

## Methods

- `from`
- `to`
- `from`
- `to`
- `center`
- `radius`
- `center`
- `radius`
- `start_deg`
- `end_deg`
- `position`
- `content`
- `size`
- `vertex_count`

## Source
Lines 193–222 in `crates/oxide-app/src/panels/symbol_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_context](/crates/oxide-app/src/panels/symbol_context.md) |
