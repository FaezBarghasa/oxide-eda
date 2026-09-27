---
okf_version: "0.2"
type: Function
title: build_pin_net_lookup
description: "Reads net names straight off the authoritative `Netlist` (ADR-0002 D7)"
resource: crates/oxide-output/src/expression.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:16Z"
concept_id: crates/oxide-output/src/expression/build_pin_net_lookup
language: rust
---

# build_pin_net_lookup

Reads net names straight off the authoritative `Netlist` (ADR-0002 D7)

## Signature

```rust
fn build_pin_net_lookup(netlist: Option<&Netlist>) -> HashMap<String, HashMap<String, String>>
```

## Docstring

Reads net names straight off the authoritative `Netlist` (ADR-0002 D7)
instead of re-deriving connectivity: one `HashMap` walk over the
netlist's terminals, keyed by symbol uuid then pin number. `None` (no
netlist attached to this export) yields an empty table rather than a
guess — the whole point of reading the contract instead of re-deriving
it is that "unknown" stays unknown.

## Source
Lines 82–98 in `crates/oxide-output/src/expression.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expression](/crates/oxide-output/src/expression.md) |
| called_by | [build_expression_tables](/crates/oxide-output/src/expression/build_expression_tables.md) |
| called_by | [pin_net_lookup_reads_names_from_the_netlist](/crates/oxide-output/src/expression/pin_net_lookup_reads_names_from_the_netlist.md) |
