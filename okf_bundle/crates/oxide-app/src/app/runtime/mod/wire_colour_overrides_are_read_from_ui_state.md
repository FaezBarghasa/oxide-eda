---
okf_version: "0.2"
type: Function
title: wire_colour_overrides_are_read_from_ui_state
description: "The wire-colour overrides are borrowed, not cloned onto the canvas."
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/wire_colour_overrides_are_read_from_ui_state
language: rust
---

# wire_colour_overrides_are_read_from_ui_state

The wire-colour overrides are borrowed, not cloned onto the canvas.

## Signature

```rust
fn wire_colour_overrides_are_read_from_ui_state()
```

## Decorators

- `test`

## Docstring

The wire-colour overrides are borrowed, not cloned onto the canvas.
The old copy was cleared in parallel with `ui_state`'s at each
mutation site — one missed site and the canvas kept painting a
colour the user had cleared.
[test]

## Source
Lines 366–395 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
