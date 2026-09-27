---
okf_version: "0.2"
type: Module
title: tests
resource: crates/oxide-erc/src/rules/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/tests
language: rust
---

# tests

## Relationships

| Type | Target |
|------|--------|
| related | [pt](/crates/oxide-erc/src/rules/tests/pt.md) |
| related | [wire](/crates/oxide-erc/src/rules/tests/wire.md) |
| related | [power_label](/crates/oxide-erc/src/rules/tests/power_label.md) |
| related | [power_port](/crates/oxide-erc/src/rules/tests/power_port.md) |
| related | [net_label](/crates/oxide-erc/src/rules/tests/net_label.md) |
| related | [global_label](/crates/oxide-erc/src/rules/tests/global_label.md) |
| related | [symbol_with_pin](/crates/oxide-erc/src/rules/tests/symbol_with_pin.md) |
| related | [ctx](/crates/oxide-erc/src/rules/tests/ctx.md) |
| related | [missing_power_flag_honors_a_t_junction](/crates/oxide-erc/src/rules/tests/missing_power_flag_honors_a_t_junction.md) |
| related | [missing_power_flag_fires_when_no_same_net_label](/crates/oxide-erc/src/rules/tests/missing_power_flag_fires_when_no_same_net_label.md) |
| related | [unused_pin_trusts_a_through_junction_tap](/crates/oxide-erc/src/rules/tests/unused_pin_trusts_a_through_junction_tap.md) |
| related | [unused_pin_still_fires_when_not_connected](/crates/oxide-erc/src/rules/tests/unused_pin_still_fires_when_not_connected.md) |
| related | [net_label_conflict_catches_wire_interior_labels](/crates/oxide-erc/src/rules/tests/net_label_conflict_catches_wire_interior_labels.md) |
| related | [orphan_label_accepts_a_power_label_on_a_wire_interior](/crates/oxide-erc/src/rules/tests/orphan_label_accepts_a_power_label_on_a_wire_interior.md) |
| related | [hier_port_disconnected_accepts_a_global_label_on_a_wire_interior](/crates/oxide-erc/src/rules/tests/hier_port_disconnected_accepts_a_global_label_on_a_wire_interior.md) |
| related | [missing_power_flag_accepts_a_mid_wire_cross_ref_label](/crates/oxide-erc/src/rules/tests/missing_power_flag_accepts_a_mid_wire_cross_ref_label.md) |
| related | [net_label_conflict_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/net_label_conflict_is_independent_of_label_order.md) |
| related | [missing_power_flag_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/missing_power_flag_is_independent_of_label_order.md) |
| related | [bus_ctx](/crates/oxide-erc/src/rules/tests/bus_ctx.md) |
| related | [bus](/crates/oxide-erc/src/rules/tests/bus.md) |
| related | [bus_bit_width_mismatch_catches_mid_bus_range_labels](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_catches_mid_bus_range_labels.md) |
| related | [bus_bit_width_mismatch_still_catches_endpoint_range_labels](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_still_catches_endpoint_range_labels.md) |
| related | [bus_bit_width_mismatch_accepts_matching_mid_bus_widths](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_accepts_matching_mid_bus_widths.md) |
| related | [bus_bit_width_mismatch_keeps_separate_buses_apart](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_keeps_separate_buses_apart.md) |
| related | [bus_bit_width_mismatch_ignores_a_label_off_every_bus](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_ignores_a_label_off_every_bus.md) |
| related | [net_label_conflict_sees_a_join_made_by_a_global_label](/crates/oxide-erc/src/rules/tests/net_label_conflict_sees_a_join_made_by_a_global_label.md) |
| related | [missing_power_flag_honors_a_third_label_name_merge](/crates/oxide-erc/src/rules/tests/missing_power_flag_honors_a_third_label_name_merge.md) |
| related | [missing_power_flag_still_fires_on_a_bare_net_beside_a_merged_pair](/crates/oxide-erc/src/rules/tests/missing_power_flag_still_fires_on_a_bare_net_beside_a_merged_pair.md) |
