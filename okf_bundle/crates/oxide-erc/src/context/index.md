# context

## Classs

- [ErcBus](ErcBus.md) — [derive(Debug, Clone, Copy)]
- [ErcBusEntry](ErcBusEntry.md) — [derive(Debug, Clone, Copy)]
- [ErcChildSheet](ErcChildSheet.md) — [derive(Debug, Clone)]
- [ErcContext](ErcContext.md) — Normalised, render-independent view of a schematic sheet for ERC purposes.
- [ErcJunction](ErcJunction.md) — [derive(Debug, Clone, Copy)]
- [ErcLabel](ErcLabel.md) — [derive(Debug, Clone)]
- [ErcNet](ErcNet.md) — A logical net: the set of pins and labels connected by wires/junctions.
- [ErcNoConnect](ErcNoConnect.md) — [derive(Debug, Clone, Copy)]
- [ErcPin](ErcPin.md) — A single pin instance in world-space, ready for rule evaluation.
- [ErcSheetPin](ErcSheetPin.md) — [derive(Debug, Clone)]
- [ErcSymbol](ErcSymbol.md) — A placed component instance. Its `pins` are already transformed to
- [ErcWire](ErcWire.md) — [derive(Debug, Clone, Copy)]
- [PaperSize](PaperSize.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]

## Functions

- [a_merged_net_takes_the_highest_priority_label_name](a_merged_net_takes_the_highest_priority_label_name.md) — [test]
- [differently_named_labels_do_not_merge](differently_named_labels_do_not_merge.md) — [test]
- [dimensions_mm](dimensions_mm.md) — Returns `(width_mm, height_mm)` for **landscape** orientation
- [dimensions_mm](dimensions_mm_1.md) — Returns `(width_mm, height_mm)` for **landscape** orientation
- [endpoint_label_still_names_its_net](endpoint_label_still_names_its_net.md) — [test]
- [from_snapshot](from_snapshot.md)
- [from_snapshot](from_snapshot_1.md)
- [from_snapshot_with_children](from_snapshot_with_children.md) — `resolved` is THIS sheet's own resolution submap — `cs.filename ->
- [from_snapshot_with_children](from_snapshot_with_children_1.md) — `resolved` is THIS sheet's own resolution submap — `cs.filename ->
- [junction_gates_connectivity_but_off_points_do_not](junction_gates_connectivity_but_off_points_do_not.md) — [test]
- [label](label.md)
- [label_off_every_wire_does_not_join_a_net](label_off_every_wire_does_not_join_a_net.md) — [test]
- [mid_wire_label_names_the_net_the_dsl_reads](mid_wire_label_names_the_net_the_dsl_reads.md) — [test]
- [parse](parse.md)
- [parse](parse_1.md)
- [pin](pin.md)
- [point_is_connected](point_is_connected.md) — True when a wire endpoint, junction, label, or no-connect sits at `pos`,
- [project](project.md)
- [project](project_1.md)
- [pt](pt.md)
- [pt_key](pt_key.md) — MD-6: see `rules::key` — the 1 µm bucket, the single "same point" metric so
- [same_name_hierarchical_labels_do_not_merge](same_name_hierarchical_labels_do_not_merge.md) — [test]
- [same_name_labels_merge_disjoint_wires_into_one_net](same_name_labels_merge_disjoint_wires_into_one_net.md) — [test]
- [summarize_nets](summarize_nets.md) — Summarise each geometric net into the flat [`ErcNet`] the ERC DSL reads:
- [symbol](symbol.md)
- [t_intersection_without_junction_stays_two_nets](t_intersection_without_junction_stays_two_nets.md) — [test]
- [t_junction_merges_wire_ending_on_another_wires_interior](t_junction_merges_wire_ending_on_another_wires_interior.md) — [test]
- [wire](wire.md)
