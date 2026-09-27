---
okf_version: "0.2"
type: Function
title: arc_symbol
description: "--- Arc CCW-wraparound convention ---------------------------------------"
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/arc_symbol
language: rust
---

# arc_symbol

--- Arc CCW-wraparound convention ---------------------------------------

## Signature

```rust
fn arc_symbol(start_deg: f64, end_deg: f64) -> Symbol
```

## Docstring

--- Arc CCW-wraparound convention ---------------------------------------

## Source
Lines 817–831 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| called_by | [arc_endpoint_handle_drag_survives_save_reload](/crates/oxide-app/src/library/editor/symbol/state/tests/arc_endpoint_handle_drag_survives_save_reload.md) |
| called_by | [rotated_wraparound_arc_hit_test_and_draw_sweep_agree](/crates/oxide-app/src/library/editor/symbol/state/tests/rotated_wraparound_arc_hit_test_and_draw_sweep_agree.md) |
