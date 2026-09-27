---
okf_version: "0.2"
type: Function
title: on_any_bus_endpoint
description: "True when `pos` sits on a bus **endpoint** — buses are member bundles, not"
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/on_any_bus_endpoint
language: rust
---

# on_any_bus_endpoint

True when `pos` sits on a bus **endpoint** — buses are member bundles, not

## Signature

```rust
fn on_any_bus_endpoint(pos: &Point, ctx: &ErcContext) -> bool
```

## Docstring

True when `pos` sits on a bus **endpoint** — buses are member bundles, not
single nets, so (D5.4) they deliberately never get interior anchoring;
only the "same point" metric changes here, from float-epsilon `same()` to
the canonical 1 µm `pt_key` (D5.5).

## Source
Lines 62–67 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [pt_key](/crates/oxide-erc/src/context/pt_key.md) |
| called_by | [hier_port_disconnected](/crates/oxide-erc/src/rules/mod/hier_port_disconnected.md) |
| called_by | [orphan_label](/crates/oxide-erc/src/rules/mod/orphan_label.md) |
