---
okf_version: "0.2"
type: Function
title: run_all
description: "Run every built-in rule against `ctx` and return all diagnostics."
resource: crates/oxide-erc/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc/src/engine/run_all
language: rust
---

# run_all

Run every built-in rule against `ctx` and return all diagnostics.

## Signature

```rust
pub fn run_all(ctx: &ErcContext) -> Vec<Diagnostic>
```

## Visibility

- `pub`

## Docstring

Run every built-in rule against `ctx` and return all diagnostics.

## Source
Lines 18–33 in `crates/oxide-erc/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-erc/src/engine.md) |
| calls | [unused_pin](/crates/oxide-erc/src/rules/mod/unused_pin.md) |
| calls | [duplicate_ref_designator](/crates/oxide-erc/src/rules/mod/duplicate_ref_designator.md) |
| calls | [hier_port_disconnected](/crates/oxide-erc/src/rules/mod/hier_port_disconnected.md) |
| calls | [dangling_wire](/crates/oxide-erc/src/rules/mod/dangling_wire.md) |
| calls | [net_label_conflict](/crates/oxide-erc/src/rules/mod/net_label_conflict.md) |
| calls | [orphan_label](/crates/oxide-erc/src/rules/mod/orphan_label.md) |
| calls | [bus_bit_width_mismatch](/crates/oxide-erc/src/rules/mod/bus_bit_width_mismatch.md) |
| calls | [bad_hier_sheet_pin](/crates/oxide-erc/src/rules/mod/bad_hier_sheet_pin.md) |
| calls | [missing_power_flag](/crates/oxide-erc/src/rules/mod/missing_power_flag.md) |
| calls | [power_port_short](/crates/oxide-erc/src/rules/mod/power_port_short.md) |
| calls | [symbol_outside_sheet](/crates/oxide-erc/src/rules/mod/symbol_outside_sheet.md) |
| calls | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
| called_by | [run_all_with_dsl](/crates/oxide-erc/src/engine/run_all_with_dsl.md) |
| called_by | [run](/crates/oxide-erc/src/lib/run.md) |
| called_by | [run_with_project](/crates/oxide-erc/src/lib/run_with_project.md) |
