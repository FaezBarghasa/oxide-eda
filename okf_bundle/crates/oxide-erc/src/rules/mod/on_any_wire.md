---
okf_version: "0.2"
type: Function
title: on_any_wire
description: "True when `pos` sits on any wire's segment — endpoint **or interior** —"
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/on_any_wire
language: rust
---

# on_any_wire

True when `pos` sits on any wire's segment — endpoint **or interior** —

## Signature

```rust
fn on_any_wire(pos: &Point, wires: &[(Point, Point)]) -> bool
```

## Docstring

True when `pos` sits on any wire's segment — endpoint **or interior** —
via the shared [`point_on_segment`], the same anchoring `build_netlist`
applies to labels (issue #388). Replaces the endpoint-only `same()` gate
that missed mid-wire label/pin placements.

## Source
Lines 51–56 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [pt_key](/crates/oxide-erc/src/context/pt_key.md) |
| called_by | [hier_port_disconnected](/crates/oxide-erc/src/rules/mod/hier_port_disconnected.md) |
| called_by | [orphan_label](/crates/oxide-erc/src/rules/mod/orphan_label.md) |
