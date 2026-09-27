---
okf_version: "0.2"
type: Module
title: eco
description: Bidirectional Engineering Change Order (ECO) Engine.
resource: crates/oxide-net/src/eco.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:13:59Z"
concept_id: crates/oxide-net/src/eco
language: rust
---

# eco

Bidirectional Engineering Change Order (ECO) Engine.

## Docstring

Bidirectional Engineering Change Order (ECO) Engine.

Performs deep structural diffing between the authoritative schematic Netlist
and physical PCB layout state, synthesizing atomic ECO actions (add/remove/rename nets,
add/remove footprints, re-assign pins/pads, update designators).

## Relationships

| Type | Target |
|------|--------|
| related | [EcoAction](/crates/oxide-net/src/eco/EcoAction.md) |
| related | [EcoReport](/crates/oxide-net/src/eco/EcoReport.md) |
| related | [is_empty](/crates/oxide-net/src/eco/is_empty.md) |
| related | [len](/crates/oxide-net/src/eco/len.md) |
| related | [is_empty](/crates/oxide-net/src/eco/is_empty.md) |
| related | [len](/crates/oxide-net/src/eco/len.md) |
| related | [EcoEngine](/crates/oxide-net/src/eco/EcoEngine.md) |
| related | [diff_schematic_to_pcb](/crates/oxide-net/src/eco/diff_schematic_to_pcb.md) |
| related | [apply_eco](/crates/oxide-net/src/eco/apply_eco.md) |
| related | [diff_schematic_to_pcb](/crates/oxide-net/src/eco/diff_schematic_to_pcb.md) |
| related | [apply_eco](/crates/oxide-net/src/eco/apply_eco.md) |
| related | [test_eco_diff_and_apply](/crates/oxide-net/src/eco/test_eco_diff_and_apply.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
