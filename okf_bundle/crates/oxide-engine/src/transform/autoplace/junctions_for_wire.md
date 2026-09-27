---
okf_version: "0.2"
type: Function
title: junctions_for_wire
description: "Every junction dot the sheet needs on account of `wire` — **both**"
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/junctions_for_wire
language: rust
---

# junctions_for_wire

Every junction dot the sheet needs on account of `wire` — **both**

## Signature

```rust
pub(crate) fn junctions_for_wire(
    wire: &oxide_types::schematic::Wire,
    document: &SchematicSheet,
    tolerance: f64,
) -> Vec<oxide_types::schematic::Junction>
```

## Visibility

- `pub(crate)`

## Docstring

Every junction dot the sheet needs on account of `wire` — **both**
directions of the T: the wire's own endpoints landing on something
([`needed_junction`]) and something else's endpoint landing on this wire's
interior ([`junctions_under_new_wire`]).

Call with `wire` already present in `document`. Any command that creates or
moves wire geometry must route through here; reconciling only the placement
path leaves drag / rotate / mirror minting junction-less Ts (issue #402).

## Source
Lines 381–394 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| calls | [needed_junction](/crates/oxide-engine/src/transform/autoplace/needed_junction.md) |
| calls | [junctions_under_new_wire](/crates/oxide-engine/src/transform/autoplace/junctions_under_new_wire.md) |
| called_by | [reconcile_wire_junctions](/crates/oxide-engine/src/transform/mod/reconcile_wire_junctions.md) |
