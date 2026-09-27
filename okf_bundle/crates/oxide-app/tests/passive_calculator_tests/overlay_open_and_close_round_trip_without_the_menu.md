---
okf_version: "0.2"
type: Function
title: overlay_open_and_close_round_trip_without_the_menu
description: "Open and close are both `OverlayMsg` leaves, so the pair can be driven"
resource: crates/oxide-app/tests/passive_calculator_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/passive_calculator_tests/overlay_open_and_close_round_trip_without_the_menu
language: rust
---

# overlay_open_and_close_round_trip_without_the_menu

Open and close are both `OverlayMsg` leaves, so the pair can be driven

## Signature

```rust
fn overlay_open_and_close_round_trip_without_the_menu()
```

## Decorators

- `test`

## Docstring

Open and close are both `OverlayMsg` leaves, so the pair can be driven
without going through the menu at all — the route a keybinding or the
command registry would take.
[test]

## Source
Lines 56–69 in `crates/oxide-app/tests/passive_calculator_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [passive_calculator_tests](/crates/oxide-app/tests/passive_calculator_tests.md) |
