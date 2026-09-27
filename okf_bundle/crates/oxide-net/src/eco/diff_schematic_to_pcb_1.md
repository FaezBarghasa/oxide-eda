---
okf_version: "0.2"
type: Function
title: diff_schematic_to_pcb
description: "Generate Forward ECO (Schematic -> PCB)."
resource: crates/oxide-net/src/eco.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:13:59Z"
concept_id: crates/oxide-net/src/eco/diff_schematic_to_pcb_1
language: rust
---

# diff_schematic_to_pcb

Generate Forward ECO (Schematic -> PCB).

## Signature

```rust
pub fn diff_schematic_to_pcb(
        netlist: &Netlist,
        board: &PcbBoard,
        schematic_components: &[(String, String, String)], // (reference, value, footprint_id)
    ) -> EcoReport
```

## Visibility

- `pub`

## Docstring

Generate Forward ECO (Schematic -> PCB).
Computes changes required on the PCB to match the schematic netlist and symbols.

## Source
Lines 91–205 in `crates/oxide-net/src/eco.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eco](/crates/oxide-net/src/eco.md) |
