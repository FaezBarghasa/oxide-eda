---
okf_version: "0.2"
type: Function
title: arc_symbol
description: "---- Arc CCW-wraparound load migration ----"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/arc_symbol
language: rust
---

# arc_symbol

---- Arc CCW-wraparound load migration ----

## Signature

```rust
fn arc_symbol(start_deg: f64, end_deg: f64) -> Symbol
```

## Docstring

---- Arc CCW-wraparound load migration ----

## Source
Lines 568–582 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
| called_by | [arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load.md) |
| called_by | [arc_migration_is_a_load_time_fixed_point](/crates/oxide-library/src/primitive/symbol/tests/arc_migration_is_a_load_time_fixed_point.md) |
| called_by | [arc_near_full_turn_stays_an_arc_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_near_full_turn_stays_an_arc_on_load.md) |
| called_by | [arc_negative_full_turn_migrates_to_circle_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_negative_full_turn_migrates_to_circle_on_load.md) |
| called_by | [arc_wraparound_pair_already_in_range_is_unchanged_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_wraparound_pair_already_in_range_is_unchanged_on_load.md) |
