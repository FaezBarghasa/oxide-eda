---
okf_version: "0.2"
type: Class
title: GraphicFieldId
description: Identifier for one numeric field on a graphic — carried by
resource: crates/oxide-app/src/panels/symbol_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/symbol_context/GraphicFieldId
language: rust
---

# GraphicFieldId

Identifier for one numeric field on a graphic — carried by

## Signature

```rust
pub enum GraphicFieldId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Identifier for one numeric field on a graphic — carried by
[`PanelMsg::SymEditorSetGraphicField`] so the dispatcher knows which
scalar to mutate. The dispatcher silently ignores (idx, field)
pairs whose field doesn't apply to the graphic's kind.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 229–256 in `crates/oxide-app/src/panels/symbol_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_context](/crates/oxide-app/src/panels/symbol_context.md) |
