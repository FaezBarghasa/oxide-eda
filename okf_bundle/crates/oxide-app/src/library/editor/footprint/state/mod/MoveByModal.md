---
okf_version: "0.2"
type: Class
title: MoveByModal
description: "v0.14 — typed-delta \"Move Selection By X, Y\" modal. `None` on"
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/MoveByModal
language: rust
---

# MoveByModal

v0.14 — typed-delta "Move Selection By X, Y" modal. `None` on

## Signature

```rust
pub struct MoveByModal
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq)`

## Visibility

- `pub`

## Docstring

v0.14 — typed-delta "Move Selection By X, Y" modal. `None` on
`FootprintEditorState::move_by_modal` means the modal is closed.
Two erasable string buffers (same pattern as `dimension_input`) so
typing "-" / "." mid-entry doesn't fight an f64 binding.
[derive(Debug, Clone, Default, PartialEq)]

## Methods

- `dx_buf`
- `dy_buf`

## Source
Lines 62–65 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
