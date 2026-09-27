---
okf_version: "0.2"
type: Function
title: next_pin_number
description: Pick the next integer pin number — one above the highest numeric
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/next_pin_number
language: rust
---

# next_pin_number

Pick the next integer pin number — one above the highest numeric

## Signature

```rust
fn next_pin_number(sym: &Symbol) -> String
```

## Docstring

Pick the next integer pin number — one above the highest numeric
pin number, or `"1"` if no numeric pins exist.

## Source
Lines 557–565 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [add_pin](/crates/oxide-app/src/library/editor/symbol/state/mod/add_pin.md) |
