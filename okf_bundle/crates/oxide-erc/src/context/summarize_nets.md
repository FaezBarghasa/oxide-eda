---
okf_version: "0.2"
type: Function
title: summarize_nets
description: "Summarise each geometric net into the flat [`ErcNet`] the ERC DSL reads:"
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/summarize_nets
language: rust
---

# summarize_nets

Summarise each geometric net into the flat [`ErcNet`] the ERC DSL reads:

## Signature

```rust
fn summarize_nets(
    wires: &[ErcWire],
    labels: &[ErcLabel],
    junctions: &[ErcJunction],
    symbols: &[ErcSymbol],
) -> Vec<ErcNet>
```

## Docstring

Summarise each geometric net into the flat [`ErcNet`] the ERC DSL reads:
its highest-priority label name, a coarse class, and the electrical types of
the pins on it.

Connectivity is **not** derived here — it comes from the shared
[`SheetConnectivity`]: the physical wire-endpoint union plus junction
T-merge, *and* the logical same-name label merge on top
([`SheetConnectivity::merge_named_labels`]). Both halves are the ones
`build_netlist` applies, so ERC and the netlist agree on membership by
construction instead of ERC hand-rolling a second, thinner union-find.

## Source
Lines 409–513 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
| called_by | [a_merged_net_takes_the_highest_priority_label_name](/crates/oxide-erc/src/context/a_merged_net_takes_the_highest_priority_label_name.md) |
| called_by | [differently_named_labels_do_not_merge](/crates/oxide-erc/src/context/differently_named_labels_do_not_merge.md) |
| called_by | [endpoint_label_still_names_its_net](/crates/oxide-erc/src/context/endpoint_label_still_names_its_net.md) |
| called_by | [label_off_every_wire_does_not_join_a_net](/crates/oxide-erc/src/context/label_off_every_wire_does_not_join_a_net.md) |
| called_by | [mid_wire_label_names_the_net_the_dsl_reads](/crates/oxide-erc/src/context/mid_wire_label_names_the_net_the_dsl_reads.md) |
| called_by | [project](/crates/oxide-erc/src/context/project.md) |
| called_by | [same_name_hierarchical_labels_do_not_merge](/crates/oxide-erc/src/context/same_name_hierarchical_labels_do_not_merge.md) |
| called_by | [same_name_labels_merge_disjoint_wires_into_one_net](/crates/oxide-erc/src/context/same_name_labels_merge_disjoint_wires_into_one_net.md) |
| called_by | [t_intersection_without_junction_stays_two_nets](/crates/oxide-erc/src/context/t_intersection_without_junction_stays_two_nets.md) |
| called_by | [t_junction_merges_wire_ending_on_another_wires_interior](/crates/oxide-erc/src/context/t_junction_merges_wire_ending_on_another_wires_interior.md) |
