---
okf_version: "0.2"
type: Function
title: bus_bit_width_mismatch
description: "---------------------------------------------------------------------------"
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/bus_bit_width_mismatch
language: rust
---

# bus_bit_width_mismatch

---------------------------------------------------------------------------

## Signature

```rust
pub(crate) fn bus_bit_width_mismatch(ctx: &ErcContext, out: &mut Vec<Diagnostic>)
```

## Visibility

- `pub(crate)`

## Docstring

---------------------------------------------------------------------------
Rule: BusBitWidthMismatch
---------------------------------------------------------------------------

## Source
Lines 318–414 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [parse_bus_label](/crates/oxide-erc/src/rules/mod/parse_bus_label.md) |
| calls | [sel](/crates/oxide-erc/src/lib/sel.md) |
| called_by | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [bus_bit_width_mismatch_accepts_matching_mid_bus_widths](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_accepts_matching_mid_bus_widths.md) |
| called_by | [bus_bit_width_mismatch_catches_mid_bus_range_labels](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_catches_mid_bus_range_labels.md) |
| called_by | [bus_bit_width_mismatch_ignores_a_label_off_every_bus](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_ignores_a_label_off_every_bus.md) |
| called_by | [bus_bit_width_mismatch_keeps_separate_buses_apart](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_keeps_separate_buses_apart.md) |
| called_by | [bus_bit_width_mismatch_still_catches_endpoint_range_labels](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_still_catches_endpoint_range_labels.md) |
