---
okf_version: "0.2"
type: Function
title: sym_editor_mutate_pin
description: "Helper — apply a closure to the pin at `pin_idx` on the active"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_mutate_pin
language: rust
---

# sym_editor_mutate_pin

Helper — apply a closure to the pin at `pin_idx` on the active

## Signature

```rust
impl Oxide { pub(super) fn sym_editor_mutate_pin(&mut self, pin_idx: usize, mutator: F) -> bool }
```

## Type Parameters

- `F`

## Visibility

- `pub(super)`

## Docstring

Helper — apply a closure to the pin at `pin_idx` on the active
Symbol editor and run the standard dirty/refresh cycle. Returns
silently when no Symbol editor is active or the index is out of
range so callers don't have to gate the call with their own
match.

## Source
Lines 48–64 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
