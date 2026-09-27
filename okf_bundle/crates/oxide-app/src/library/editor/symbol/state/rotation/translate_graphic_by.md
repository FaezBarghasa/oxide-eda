---
okf_version: "0.2"
type: Function
title: translate_graphic_by
description: "Shift a single graphic kind by `(dx, dy)` mm in-place."
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/translate_graphic_by
language: rust
---

# translate_graphic_by

Shift a single graphic kind by `(dx, dy)` mm in-place.

## Signature

```rust
pub fn translate_graphic_by(kind: &mut SymbolGraphicKind, dx: f64, dy: f64)
```

## Visibility

- `pub`

## Docstring

Shift a single graphic kind by `(dx, dy)` mm in-place.

## Source
Lines 313–342 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| called_by | [move_all](/crates/oxide-app/src/library/editor/symbol/state/movement/move_all.md) |
| called_by | [move_multiple](/crates/oxide-app/src/library/editor/symbol/state/movement/move_multiple.md) |
