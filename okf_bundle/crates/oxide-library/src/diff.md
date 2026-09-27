---
okf_version: "0.2"
type: Module
title: diff
description: Pure-data diff between two rows of the same component table.
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff
language: rust
---

# diff

Pure-data diff between two rows of the same component table.

## Docstring

Pure-data diff between two rows of the same component table.

Per `v0.9-refactor-2-plan.md` §6 step 1.7, the diff now operates on
[`ComponentRow`] pairs — geometry-level diffs (pin moves, pad changes)
live with the primitive editors and are out of scope here. This crate's
[`RowDiff`] answers:
"did the symbol primitive UUID change? did the MPN swap? did supply
listings move? did the datasheet repoint?"

The diff is the data backbone for:
* the visual diff renderer (drawn by oxide-app — out of scope here),
* the auto-bump heuristic — call [`auto_bump_kind`] to decide whether a
save should be tagged as a small or large change.

## Relationships

| Type | Target |
|------|--------|
| related | [RowDiff](/crates/oxide-library/src/diff/RowDiff.md) |
| related | [ParameterDiff](/crates/oxide-library/src/diff/ParameterDiff.md) |
| related | [PinMapDiff](/crates/oxide-library/src/diff/PinMapDiff.md) |
| related | [ListDiff](/crates/oxide-library/src/diff/ListDiff.md) |
| related | [LifecycleDiff](/crates/oxide-library/src/diff/LifecycleDiff.md) |
| related | [BumpKind](/crates/oxide-library/src/diff/BumpKind.md) |
| related | [auto_bump_kind](/crates/oxide-library/src/diff/auto_bump_kind.md) |
| related | [diff_rows](/crates/oxide-library/src/diff/diff_rows.md) |
| related | [mpn_key](/crates/oxide-library/src/diff/mpn_key.md) |
| related | [diff_parameters](/crates/oxide-library/src/diff/diff_parameters.md) |
| related | [diff_pin_map](/crates/oxide-library/src/diff/diff_pin_map.md) |
| related | [diff_alternates](/crates/oxide-library/src/diff/diff_alternates.md) |
| related | [diff_supply](/crates/oxide-library/src/diff/diff_supply.md) |
| related | [diff_lifecycle](/crates/oxide-library/src/diff/diff_lifecycle.md) |
| related | [row](/crates/oxide-library/src/diff/row.md) |
| related | [diff_detects_symbol_ref_change](/crates/oxide-library/src/diff/diff_detects_symbol_ref_change.md) |
| related | [diff_detects_footprint_ref_change](/crates/oxide-library/src/diff/diff_detects_footprint_ref_change.md) |
| related | [diff_detects_sim_ref_change](/crates/oxide-library/src/diff/diff_detects_sim_ref_change.md) |
| related | [diff_detects_mpn_change](/crates/oxide-library/src/diff/diff_detects_mpn_change.md) |
| related | [diff_detects_pin_map_added_and_changed](/crates/oxide-library/src/diff/diff_detects_pin_map_added_and_changed.md) |
| related | [diff_detects_parameter_added_removed_changed](/crates/oxide-library/src/diff/diff_detects_parameter_added_removed_changed.md) |
| related | [diff_detects_supply_added](/crates/oxide-library/src/diff/diff_detects_supply_added.md) |
| related | [diff_detects_alternates_added](/crates/oxide-library/src/diff/diff_detects_alternates_added.md) |
| related | [diff_detects_datasheet_change](/crates/oxide-library/src/diff/diff_detects_datasheet_change.md) |
| related | [diff_detects_state_change](/crates/oxide-library/src/diff/diff_detects_state_change.md) |
| related | [auto_bump_minor_when_only_metadata_changes](/crates/oxide-library/src/diff/auto_bump_minor_when_only_metadata_changes.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
