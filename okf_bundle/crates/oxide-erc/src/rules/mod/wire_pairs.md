---
okf_version: "0.2"
type: Function
title: wire_pairs
description: "Every wire on the sheet as a `(start, end)` pair, for anchoring queries"
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/wire_pairs
language: rust
---

# wire_pairs

Every wire on the sheet as a `(start, end)` pair, for anchoring queries

## Signature

```rust
pub(super) fn wire_pairs(ctx: &ErcContext) -> Vec<(Point, Point)>
```

## Visibility

- `pub(super)`

## Docstring

Every wire on the sheet as a `(start, end)` pair, for anchoring queries
against [`SheetConnectivity::root_of_anchored`] / [`on_any_wire`].

## Source
Lines 33–35 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| called_by | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
| called_by | [hier_port_disconnected](/crates/oxide-erc/src/rules/mod/hier_port_disconnected.md) |
| called_by | [missing_power_flag](/crates/oxide-erc/src/rules/mod/missing_power_flag.md) |
| called_by | [net_label_conflict](/crates/oxide-erc/src/rules/mod/net_label_conflict.md) |
| called_by | [orphan_label](/crates/oxide-erc/src/rules/mod/orphan_label.md) |
| called_by | [wire_connectivity](/crates/oxide-erc/src/rules/mod/wire_connectivity.md) |
