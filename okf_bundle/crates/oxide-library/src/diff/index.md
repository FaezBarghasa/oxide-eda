# diff

## Classs

- [BumpKind](BumpKind.md) — Auto-bump heuristic — `Major` when the binding shape changes (symbol,
- [LifecycleDiff](LifecycleDiff.md) — [derive(Clone, Debug, Default, PartialEq)]
- [ListDiff](ListDiff.md) — Identity-keyed diff over a list — `String` keys (e.g. `manufacturer:mpn`).
- [ParameterDiff](ParameterDiff.md) — [derive(Clone, Debug, Default, PartialEq)]
- [PinMapDiff](PinMapDiff.md) — [derive(Clone, Debug, Default, PartialEq)]
- [RowDiff](RowDiff.md) — Field-level changed flags + grouped detail. Mirrors plan §6 step 1.7.

## Functions

- [auto_bump_kind](auto_bump_kind.md) — Decide whether the change between two rows is a `Minor` or `Major`
- [auto_bump_minor_when_only_metadata_changes](auto_bump_minor_when_only_metadata_changes.md) — [test]
- [diff_alternates](diff_alternates.md)
- [diff_detects_alternates_added](diff_detects_alternates_added.md) — [test]
- [diff_detects_datasheet_change](diff_detects_datasheet_change.md) — [test]
- [diff_detects_footprint_ref_change](diff_detects_footprint_ref_change.md) — [test]
- [diff_detects_mpn_change](diff_detects_mpn_change.md) — [test]
- [diff_detects_parameter_added_removed_changed](diff_detects_parameter_added_removed_changed.md) — [test]
- [diff_detects_pin_map_added_and_changed](diff_detects_pin_map_added_and_changed.md) — [test]
- [diff_detects_sim_ref_change](diff_detects_sim_ref_change.md) — [test]
- [diff_detects_state_change](diff_detects_state_change.md) — [test]
- [diff_detects_supply_added](diff_detects_supply_added.md) — [test]
- [diff_detects_symbol_ref_change](diff_detects_symbol_ref_change.md) — [test]
- [diff_lifecycle](diff_lifecycle.md)
- [diff_parameters](diff_parameters.md)
- [diff_pin_map](diff_pin_map.md)
- [diff_rows](diff_rows.md) — Compute the diff from `a` to `b`.
- [diff_supply](diff_supply.md)
- [mpn_key](mpn_key.md)
- [row](row.md)
