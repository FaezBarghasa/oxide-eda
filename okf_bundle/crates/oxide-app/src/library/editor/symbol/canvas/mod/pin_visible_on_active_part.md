---
okf_version: "0.2"
type: Function
title: pin_visible_on_active_part
description: "True when `pin` should render on the currently-active part."
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/pin_visible_on_active_part
language: rust
---

# pin_visible_on_active_part

True when `pin` should render on the currently-active part.

## Signature

```rust
impl SymbolCanvas<'a> { fn pin_visible_on_active_part(&self, pin: &SymbolPin) -> bool }
```

## Type Parameters

- `'a`

## Docstring

True when `pin` should render on the currently-active part.
Part Zero (`part_number == 0`) appears on every part; other
pins only render when they match `active_part`.

## Source
Lines 183–185 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
