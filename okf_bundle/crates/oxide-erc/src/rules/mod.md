---
okf_version: "0.2"
type: Module
title: rules
description: Built-in rule implementations. Each function takes a read-only
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod
language: rust
---

# rules

Built-in rule implementations. Each function takes a read-only

## Docstring

Built-in rule implementations. Each function takes a read-only
[`ErcContext`] and pushes [`Diagnostic`]s onto the accumulator.
No render or parser imports — all geometry is already world-space inside
the context.

This file is already past the size a module should reach, so new rules land
in their own sibling file rather than growing it further.

## Relationships

| Type | Target |
|------|--------|
| related | [same](/crates/oxide-erc/src/rules/mod/same.md) |
| related | [wire_pairs](/crates/oxide-erc/src/rules/mod/wire_pairs.md) |
| related | [wire_connectivity](/crates/oxide-erc/src/rules/mod/wire_connectivity.md) |
| related | [on_any_wire](/crates/oxide-erc/src/rules/mod/on_any_wire.md) |
| related | [on_any_bus_endpoint](/crates/oxide-erc/src/rules/mod/on_any_bus_endpoint.md) |
| related | [unused_pin](/crates/oxide-erc/src/rules/mod/unused_pin.md) |
| related | [duplicate_ref_designator](/crates/oxide-erc/src/rules/mod/duplicate_ref_designator.md) |
| related | [hier_port_disconnected](/crates/oxide-erc/src/rules/mod/hier_port_disconnected.md) |
| related | [dangling_wire](/crates/oxide-erc/src/rules/mod/dangling_wire.md) |
| related | [net_label_conflict](/crates/oxide-erc/src/rules/mod/net_label_conflict.md) |
| related | [orphan_label](/crates/oxide-erc/src/rules/mod/orphan_label.md) |
| related | [bus_bit_width_mismatch](/crates/oxide-erc/src/rules/mod/bus_bit_width_mismatch.md) |
| related | [parse_bus_label](/crates/oxide-erc/src/rules/mod/parse_bus_label.md) |
| related | [bad_hier_sheet_pin](/crates/oxide-erc/src/rules/mod/bad_hier_sheet_pin.md) |
| related | [missing_power_flag](/crates/oxide-erc/src/rules/mod/missing_power_flag.md) |
| related | [power_port_short](/crates/oxide-erc/src/rules/mod/power_port_short.md) |
| related | [symbol_outside_sheet](/crates/oxide-erc/src/rules/mod/symbol_outside_sheet.md) |
