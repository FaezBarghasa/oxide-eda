---
okf_version: "0.2"
type: Function
title: placement_input_line_length_pins_second_click_at_exact_distance
description: "v0.24 Track D — Line tool's second click honours the typed"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_line_length_pins_second_click_at_exact_distance
language: rust
---

# placement_input_line_length_pins_second_click_at_exact_distance

v0.24 Track D — Line tool's second click honours the typed

## Signature

```rust
fn placement_input_line_length_pins_second_click_at_exact_distance()
```

## Decorators

- `test`

## Docstring

v0.24 Track D — Line tool's second click honours the typed
`placement_input` length. With a buffer of "10" set against the
`LineLength` kind, a click that lands at `(20, 0)` must place the
line's second endpoint at exactly `(10, 0)` along the cursor's
azimuth from the first endpoint at the origin — not `(20, 0)`.

Drives the dispatcher via `Message::Library(PrimitiveEditorEvent
{ ... })` so the integration matches what the canvas + bootstrap
keyboard handler emit.
[test]

## Source
Lines 240–359 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
