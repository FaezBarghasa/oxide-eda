---
okf_version: "0.2"
type: Function
title: pin_body_delta
description: "Returns the (dx, dy) vector from the tip (connection point) to the"
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/pin_body_delta
language: rust
---

# pin_body_delta

Returns the (dx, dy) vector from the tip (connection point) to the

## Signature

```rust
pub(super) fn pin_body_delta(pin: &SymbolPin) -> (f64, f64)
```

## Visibility

- `pub(super)`

## Docstring

Returns the (dx, dy) vector from the tip (connection point) to the
body-end (symbol-body attachment point) based on orientation and length.

## Source
Lines 240–248 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| called_by | [select_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box.md) |
| called_by | [rotate_selected_with_pivot](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_with_pivot.md) |
