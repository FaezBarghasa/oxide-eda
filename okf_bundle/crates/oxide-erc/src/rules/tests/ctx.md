---
okf_version: "0.2"
type: Function
title: ctx
resource: crates/oxide-erc/src/rules/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/tests/ctx
language: rust
---

# ctx

## Signature

```rust
fn ctx(
    wires: Vec<ErcWire>,
    junctions: Vec<ErcJunction>,
    labels: Vec<ErcLabel>,
    symbols: Vec<ErcSymbol>,
) -> ErcContext
```

## Source
Lines 79–98 in `crates/oxide-erc/src/rules/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-erc/src/rules/tests.md) |
| called_by | [bus_ctx](/crates/oxide-erc/src/rules/tests/bus_ctx.md) |
| called_by | [hier_port_disconnected_accepts_a_global_label_on_a_wire_interior](/crates/oxide-erc/src/rules/tests/hier_port_disconnected_accepts_a_global_label_on_a_wire_interior.md) |
| called_by | [missing_power_flag_accepts_a_mid_wire_cross_ref_label](/crates/oxide-erc/src/rules/tests/missing_power_flag_accepts_a_mid_wire_cross_ref_label.md) |
| called_by | [missing_power_flag_fires_when_no_same_net_label](/crates/oxide-erc/src/rules/tests/missing_power_flag_fires_when_no_same_net_label.md) |
| called_by | [missing_power_flag_honors_a_t_junction](/crates/oxide-erc/src/rules/tests/missing_power_flag_honors_a_t_junction.md) |
| called_by | [missing_power_flag_honors_a_third_label_name_merge](/crates/oxide-erc/src/rules/tests/missing_power_flag_honors_a_third_label_name_merge.md) |
| called_by | [missing_power_flag_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/missing_power_flag_is_independent_of_label_order.md) |
| called_by | [missing_power_flag_still_fires_on_a_bare_net_beside_a_merged_pair](/crates/oxide-erc/src/rules/tests/missing_power_flag_still_fires_on_a_bare_net_beside_a_merged_pair.md) |
| called_by | [net_label_conflict_catches_wire_interior_labels](/crates/oxide-erc/src/rules/tests/net_label_conflict_catches_wire_interior_labels.md) |
| called_by | [net_label_conflict_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/net_label_conflict_is_independent_of_label_order.md) |
| called_by | [net_label_conflict_sees_a_join_made_by_a_global_label](/crates/oxide-erc/src/rules/tests/net_label_conflict_sees_a_join_made_by_a_global_label.md) |
| called_by | [orphan_label_accepts_a_power_label_on_a_wire_interior](/crates/oxide-erc/src/rules/tests/orphan_label_accepts_a_power_label_on_a_wire_interior.md) |
| called_by | [unused_pin_still_fires_when_not_connected](/crates/oxide-erc/src/rules/tests/unused_pin_still_fires_when_not_connected.md) |
| called_by | [unused_pin_trusts_a_through_junction_tap](/crates/oxide-erc/src/rules/tests/unused_pin_trusts_a_through_junction_tap.md) |
