---
okf_version: "0.2"
type: Function
title: hit_test
description: "Hit-test cursor world coordinates against pins, then graphic"
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test
language: rust
---

# hit_test

Hit-test cursor world coordinates against pins, then graphic

## Signature

```rust
pub fn hit_test(sym: &Symbol, x: f64, y: f64, active_part: u8) -> Option<SymbolSelection>
```

## Visibility

- `pub`

## Docstring

Hit-test cursor world coordinates against pins, then graphic
bodies. Pins win (small hit target, often inside graphics);
graphics scan in reverse so the most-recently-placed graphic
wins overlap.

## Source
Lines 61–85 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| calls | [pin_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/pin_on_part.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [graphic_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part.md) |
| calls | [hit_test_graphic_body](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_body.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
