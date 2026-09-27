# tests

## Subdirectories

- [multi_root](multi_root/index.md)

## Modules

- [multi_root](multi_root.md) — #430 — multi-root / flat-stitch traversal: every declared page the root's

## Functions

- [add_lib](add_lib.md) — A one-pin library symbol whose pin sits at local `(0, 0)`.
- [assert_equiv](assert_equiv.md) — 1 ── Equivalence gate: one root, empty resolved, is byte-for-byte
- [child_sheet](child_sheet.md)
- [cycles_are_reported_without_hanging](cycles_are_reported_without_hanging.md) — 7 ── Cycles: A→B→A and child-instantiates-root → SheetCycle, no hang.
- [duplicate_sibling_name_collision_is_suffixed](duplicate_sibling_name_collision_is_suffixed.md) — 10 ── Name collision via duplicate sibling ChildSheet.name → suffix + issue.
- [empty_sheet](empty_sheet.md)
- [equivalence_gate_root_only](equivalence_gate_root_only.md) — [test]
- [global_label_spans_two_sheets](global_label_spans_two_sheets.md) — 3 ── Global name spans sheets; power-port symbol + Power label merge.
- [junction](junction.md)
- [label](label.md)
- [lib_pin](lib_pin.md)
- [local_net_labels_do_not_cross_sheets](local_net_labels_do_not_cross_sheets.md) — 4 ── Local Net labels never cross; the two are distinct, qualified nets.
- [membership_aggregates_across_sheet_occurrences](membership_aggregates_across_sheet_occurrences.md) — [test]
- [missing_child_reported_and_local](missing_child_reported_and_local.md) — 6 ── Missing child file → issue, parent net stays local, deterministic.
- [names](names.md)
- [output_is_deterministic_across_map_order](output_is_deterministic_across_map_order.md) — 11 ── Determinism: children-map insertion order never changes the output.
- [parent_child](parent_child.md) — A parent with one child; the child has a labelled pin net. Returns
- [place](place.md)
- [place_power](place_power.md)
- [place_xform](place_xform.md)
- [power_symbol_and_power_label_merge_across_sheets](power_symbol_and_power_label_merge_across_sheets.md) — [test]
- [project_terminals_order_designators_naturally](project_terminals_order_designators_naturally.md) — [test]
- [pt](pt.md)
- [same_child_instantiated_twice_is_not_shorted](same_child_instantiated_twice_is_not_shorted.md) — 5 ── One child file instantiated twice: instances not shorted; refdes
- [same_filename_children_of_different_parents_stitch_from_their_own_files](same_filename_children_of_different_parents_stitch_from_their_own_files.md) — 12 ── #466: two parents in different directories reference a child by the
- [sheet_add_pin_lib](sheet_add_pin_lib.md) — Library whose single pin sits at `local` (for the rotated-symbol case).
- [sheet_pin](sheet_pin.md)
- [sheet_pin_anchors_to_wire_interior](sheet_pin_anchors_to_wire_interior.md) — 8 ── Sheet pin anchored on a wire *interior* on the parent.
- [sheet_pin_binds_global_child_label](sheet_pin_binds_global_child_label.md) — [test]
- [sheet_pin_binds_hierarchical_child_label](sheet_pin_binds_hierarchical_child_label.md) — 2 ── SheetPin ↔ child label binding (Hierarchical and Global); an
- [single_sheet_name_collision_matches_build_netlist_and_is_reported](single_sheet_name_collision_matches_build_netlist_and_is_reported.md) — 10b ── A bare single-sheet collision dedups through the same shared pass,
- [stitch](stitch.md) — Test-only bridge from the flat "bare filename" fixtures every test below
- [stitch_pages](stitch_pages.md) — [`stitch`] plus the flat-page roots #430 added: `pages` names further
- [two_same_name_sheet_pins_merge_through_child](two_same_name_sheet_pins_merge_through_child.md) — 9 ── Two same-name sheet pins on one ChildSheet merge through the child.
- [unmatched_sheet_pin_stays_local](unmatched_sheet_pin_stays_local.md) — [test]
- [wire](wire.md)
