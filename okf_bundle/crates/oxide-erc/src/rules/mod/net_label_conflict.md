---
okf_version: "0.2"
type: Function
title: net_label_conflict
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
concept_id: crates/oxide-erc/src/rules/mod/net_label_conflict
language: rust
---

# net_label_conflict

---------------------------------------------------------------------------

## Signature

```rust
pub(crate) fn net_label_conflict(ctx: &ErcContext, out: &mut Vec<Diagnostic>)
```

## Visibility

- `pub(crate)`

## Docstring

---------------------------------------------------------------------------
Rule: NetLabelConflict
---------------------------------------------------------------------------

## Source
Lines 214–270 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [wire_connectivity](/crates/oxide-erc/src/rules/mod/wire_connectivity.md) |
| calls | [wire_pairs](/crates/oxide-erc/src/rules/mod/wire_pairs.md) |
| calls | [sel](/crates/oxide-erc/src/lib/sel.md) |
| called_by | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [net_label_conflict_catches_wire_interior_labels](/crates/oxide-erc/src/rules/tests/net_label_conflict_catches_wire_interior_labels.md) |
| called_by | [net_label_conflict_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/net_label_conflict_is_independent_of_label_order.md) |
| called_by | [net_label_conflict_sees_a_join_made_by_a_global_label](/crates/oxide-erc/src/rules/tests/net_label_conflict_sees_a_join_made_by_a_global_label.md) |
