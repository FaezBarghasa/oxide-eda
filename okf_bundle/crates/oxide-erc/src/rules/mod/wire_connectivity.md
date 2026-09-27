---
okf_version: "0.2"
type: Function
title: wire_connectivity
description: Wire + junction net connectivity for the rules that need net roots — the
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/wire_connectivity
language: rust
---

# wire_connectivity

Wire + junction net connectivity for the rules that need net roots — the

## Signature

```rust
fn wire_connectivity(ctx: &ErcContext) -> SheetConnectivity
```

## Docstring

Wire + junction net connectivity for the rules that need net roots — the
same [`SheetConnectivity`] (wire-endpoint union plus junction T-merge)
`build_netlist` derives, so a rule's notion of "same net" matches the
netlist's. Replaces the per-rule hand-rolled union-find, which also missed
the junction T-merge (its junction loop only `find`-ed, never `union`-ed).

## Source
Lines 42–45 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [wire_pairs](/crates/oxide-erc/src/rules/mod/wire_pairs.md) |
| called_by | [missing_power_flag](/crates/oxide-erc/src/rules/mod/missing_power_flag.md) |
| called_by | [net_label_conflict](/crates/oxide-erc/src/rules/mod/net_label_conflict.md) |
