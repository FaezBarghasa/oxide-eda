---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-sim/src/deck/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:49:22Z"
concept_id: crates/oxide-sim/src/deck/mod/new
language: rust
---

# new

## Signature

```rust
impl PSpiceDeckBuilder<'a> { pub fn new(
        netlist: &'a Netlist,
        sheets: &'a [SchematicSheet],
        models: &'a HashMap<String, SimModel>,
        config: &'a SimulationConfig,
    ) -> Self }
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 22–35 in `crates/oxide-sim/src/deck/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [deck](/crates/oxide-sim/src/deck/mod.md) |
