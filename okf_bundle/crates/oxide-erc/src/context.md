---
okf_version: "0.2"
type: Module
title: context
description: Parser-independent ERC context. Built by projecting a
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context
language: rust
---

# context

Parser-independent ERC context. Built by projecting a

## Docstring

Parser-independent ERC context. Built by projecting a
[`SchematicSheet`] once; all rule functions read from here so
they stay independent from renderer internals.

## Relationships

| Type | Target |
|------|--------|
| related | [PaperSize](/crates/oxide-erc/src/context/PaperSize.md) |
| related | [dimensions_mm](/crates/oxide-erc/src/context/dimensions_mm.md) |
| related | [parse](/crates/oxide-erc/src/context/parse.md) |
| related | [dimensions_mm](/crates/oxide-erc/src/context/dimensions_mm.md) |
| related | [parse](/crates/oxide-erc/src/context/parse.md) |
| related | [ErcPin](/crates/oxide-erc/src/context/ErcPin.md) |
| related | [ErcSymbol](/crates/oxide-erc/src/context/ErcSymbol.md) |
| related | [ErcWire](/crates/oxide-erc/src/context/ErcWire.md) |
| related | [ErcBus](/crates/oxide-erc/src/context/ErcBus.md) |
| related | [ErcLabel](/crates/oxide-erc/src/context/ErcLabel.md) |
| related | [ErcJunction](/crates/oxide-erc/src/context/ErcJunction.md) |
| related | [ErcNoConnect](/crates/oxide-erc/src/context/ErcNoConnect.md) |
| related | [ErcBusEntry](/crates/oxide-erc/src/context/ErcBusEntry.md) |
| related | [ErcChildSheet](/crates/oxide-erc/src/context/ErcChildSheet.md) |
| related | [ErcSheetPin](/crates/oxide-erc/src/context/ErcSheetPin.md) |
| related | [ErcNet](/crates/oxide-erc/src/context/ErcNet.md) |
| related | [ErcContext](/crates/oxide-erc/src/context/ErcContext.md) |
| related | [from_snapshot](/crates/oxide-erc/src/context/from_snapshot.md) |
| related | [from_snapshot_with_children](/crates/oxide-erc/src/context/from_snapshot_with_children.md) |
| related | [project](/crates/oxide-erc/src/context/project.md) |
| related | [from_snapshot](/crates/oxide-erc/src/context/from_snapshot.md) |
| related | [from_snapshot_with_children](/crates/oxide-erc/src/context/from_snapshot_with_children.md) |
| related | [project](/crates/oxide-erc/src/context/project.md) |
| related | [pt_key](/crates/oxide-erc/src/context/pt_key.md) |
| related | [point_is_connected](/crates/oxide-erc/src/context/point_is_connected.md) |
| related | [summarize_nets](/crates/oxide-erc/src/context/summarize_nets.md) |
| related | [pt](/crates/oxide-erc/src/context/pt.md) |
| related | [wire](/crates/oxide-erc/src/context/wire.md) |
| related | [pin](/crates/oxide-erc/src/context/pin.md) |
| related | [symbol](/crates/oxide-erc/src/context/symbol.md) |
| related | [junction_gates_connectivity_but_off_points_do_not](/crates/oxide-erc/src/context/junction_gates_connectivity_but_off_points_do_not.md) |
| related | [t_junction_merges_wire_ending_on_another_wires_interior](/crates/oxide-erc/src/context/t_junction_merges_wire_ending_on_another_wires_interior.md) |
| related | [t_intersection_without_junction_stays_two_nets](/crates/oxide-erc/src/context/t_intersection_without_junction_stays_two_nets.md) |
| related | [label](/crates/oxide-erc/src/context/label.md) |
| related | [mid_wire_label_names_the_net_the_dsl_reads](/crates/oxide-erc/src/context/mid_wire_label_names_the_net_the_dsl_reads.md) |
| related | [endpoint_label_still_names_its_net](/crates/oxide-erc/src/context/endpoint_label_still_names_its_net.md) |
| related | [label_off_every_wire_does_not_join_a_net](/crates/oxide-erc/src/context/label_off_every_wire_does_not_join_a_net.md) |
| related | [same_name_labels_merge_disjoint_wires_into_one_net](/crates/oxide-erc/src/context/same_name_labels_merge_disjoint_wires_into_one_net.md) |
| related | [differently_named_labels_do_not_merge](/crates/oxide-erc/src/context/differently_named_labels_do_not_merge.md) |
| related | [same_name_hierarchical_labels_do_not_merge](/crates/oxide-erc/src/context/same_name_hierarchical_labels_do_not_merge.md) |
| related | [a_merged_net_takes_the_highest_priority_label_name](/crates/oxide-erc/src/context/a_merged_net_takes_the_highest_priority_label_name.md) |
