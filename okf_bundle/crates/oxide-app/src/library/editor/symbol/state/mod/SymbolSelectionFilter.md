---
okf_version: "0.2"
type: Class
title: SymbolSelectionFilter
description: v0.13 — Per-kind selectable flags for the SchLib editor.
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/SymbolSelectionFilter
language: rust
---

# SymbolSelectionFilter

v0.13 — Per-kind selectable flags for the SchLib editor.

## Signature

```rust
pub struct SymbolSelectionFilter
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

v0.13 — Per-kind selectable flags for the SchLib editor.
Mirrors the footprint editor's SelectionFilter struct.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `pins`
- `drawings`
- `texts`
- `designators`
- `values`
- `parameters`
- `other`

## Source
Lines 186–194 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
