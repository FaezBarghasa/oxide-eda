---
okf_version: "0.2"
type: Function
title: pin_on_part
description: "A pin is visible/editable on `active_part` when it is shared (Part"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/pin_on_part
language: rust
---

# pin_on_part

A pin is visible/editable on `active_part` when it is shared (Part

## Signature

```rust
pub fn pin_on_part(pin: &SymbolPin, active_part: u8) -> bool
```

## Visibility

- `pub`

## Docstring

A pin is visible/editable on `active_part` when it is shared (Part
Zero) or scoped to that exact unit — the interaction-side mirror of
`SymbolCanvas::pin_visible_on_active_part`, so click / box-select /
handle hit-tests match what the canvas actually draws.

## Source
Lines 407–409 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test.md) |
| called_by | [select_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box.md) |
