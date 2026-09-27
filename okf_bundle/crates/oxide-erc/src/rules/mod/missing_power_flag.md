---
okf_version: "0.2"
type: Function
title: missing_power_flag
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
concept_id: crates/oxide-erc/src/rules/mod/missing_power_flag
language: rust
---

# missing_power_flag

---------------------------------------------------------------------------

## Signature

```rust
pub(crate) fn missing_power_flag(ctx: &ErcContext, out: &mut Vec<Diagnostic>)
```

## Visibility

- `pub(crate)`

## Docstring

---------------------------------------------------------------------------
Rule: MissingPowerFlag
---------------------------------------------------------------------------

## Source
Lines 516–589 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [wire_connectivity](/crates/oxide-erc/src/rules/mod/wire_connectivity.md) |
| calls | [wire_pairs](/crates/oxide-erc/src/rules/mod/wire_pairs.md) |
| calls | [sel](/crates/oxide-erc/src/lib/sel.md) |
| called_by | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [missing_power_flag_accepts_a_mid_wire_cross_ref_label](/crates/oxide-erc/src/rules/tests/missing_power_flag_accepts_a_mid_wire_cross_ref_label.md) |
| called_by | [missing_power_flag_fires_when_no_same_net_label](/crates/oxide-erc/src/rules/tests/missing_power_flag_fires_when_no_same_net_label.md) |
| called_by | [missing_power_flag_honors_a_t_junction](/crates/oxide-erc/src/rules/tests/missing_power_flag_honors_a_t_junction.md) |
| called_by | [missing_power_flag_honors_a_third_label_name_merge](/crates/oxide-erc/src/rules/tests/missing_power_flag_honors_a_third_label_name_merge.md) |
| called_by | [missing_power_flag_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/missing_power_flag_is_independent_of_label_order.md) |
| called_by | [missing_power_flag_still_fires_on_a_bare_net_beside_a_merged_pair](/crates/oxide-erc/src/rules/tests/missing_power_flag_still_fires_on_a_bare_net_beside_a_merged_pair.md) |
