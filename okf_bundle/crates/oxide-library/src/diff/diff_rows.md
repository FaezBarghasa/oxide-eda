---
okf_version: "0.2"
type: Function
title: diff_rows
description: "Compute the diff from `a` to `b`."
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff/diff_rows
language: rust
---

# diff_rows

Compute the diff from `a` to `b`.

## Signature

```rust
pub fn diff_rows(a: &ComponentRow, b: &ComponentRow) -> RowDiff
```

## Visibility

- `pub`

## Docstring

Compute the diff from `a` to `b`.

## Source
Lines 103–133 in `crates/oxide-library/src/diff.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff](/crates/oxide-library/src/diff.md) |
| calls | [diff_parameters](/crates/oxide-library/src/diff/diff_parameters.md) |
| calls | [diff_pin_map](/crates/oxide-library/src/diff/diff_pin_map.md) |
| calls | [diff_alternates](/crates/oxide-library/src/diff/diff_alternates.md) |
| calls | [diff_supply](/crates/oxide-library/src/diff/diff_supply.md) |
| calls | [diff_lifecycle](/crates/oxide-library/src/diff/diff_lifecycle.md) |
| calls | [mpn_key](/crates/oxide-library/src/diff/mpn_key.md) |
| called_by | [auto_bump_minor_when_only_metadata_changes](/crates/oxide-library/src/diff/auto_bump_minor_when_only_metadata_changes.md) |
| called_by | [diff_detects_alternates_added](/crates/oxide-library/src/diff/diff_detects_alternates_added.md) |
| called_by | [diff_detects_datasheet_change](/crates/oxide-library/src/diff/diff_detects_datasheet_change.md) |
| called_by | [diff_detects_footprint_ref_change](/crates/oxide-library/src/diff/diff_detects_footprint_ref_change.md) |
| called_by | [diff_detects_mpn_change](/crates/oxide-library/src/diff/diff_detects_mpn_change.md) |
| called_by | [diff_detects_parameter_added_removed_changed](/crates/oxide-library/src/diff/diff_detects_parameter_added_removed_changed.md) |
| called_by | [diff_detects_pin_map_added_and_changed](/crates/oxide-library/src/diff/diff_detects_pin_map_added_and_changed.md) |
| called_by | [diff_detects_sim_ref_change](/crates/oxide-library/src/diff/diff_detects_sim_ref_change.md) |
| called_by | [diff_detects_state_change](/crates/oxide-library/src/diff/diff_detects_state_change.md) |
| called_by | [diff_detects_supply_added](/crates/oxide-library/src/diff/diff_detects_supply_added.md) |
| called_by | [diff_detects_symbol_ref_change](/crates/oxide-library/src/diff/diff_detects_symbol_ref_change.md) |
| called_by | [diff_is_symmetric_on_added_supply](/crates/oxide-library/tests/diff_golden/diff_is_symmetric_on_added_supply.md) |
| called_by | [lifecycle_diff_records_state_change_when_present](/crates/oxide-library/tests/diff_golden/lifecycle_diff_records_state_change_when_present.md) |
| called_by | [mpn_only_swap_is_minor_bump](/crates/oxide-library/tests/diff_golden/mpn_only_swap_is_minor_bump.md) |
| called_by | [symbol_ref_swap_is_major_bump](/crates/oxide-library/tests/diff_golden/symbol_ref_swap_is_major_bump.md) |
