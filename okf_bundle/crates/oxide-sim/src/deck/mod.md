---
okf_version: "0.2"
type: Module
title: deck
description: PSpice simulation deck generator.
resource: crates/oxide-sim/src/deck/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:49:22Z"
concept_id: crates/oxide-sim/src/deck/mod
language: rust
---

# deck

PSpice simulation deck generator.

## Docstring

PSpice simulation deck generator.

Translates schematic netlists, placed symbols, and library `SimModel`s
into a clean, valid PSpice/SPICE netlist deck.

## Relationships

| Type | Target |
|------|--------|
| related | [PSpiceDeckBuilder](/crates/oxide-sim/src/deck/mod/PSpiceDeckBuilder.md) |
| related | [new](/crates/oxide-sim/src/deck/mod/new.md) |
| related | [with_title](/crates/oxide-sim/src/deck/mod/with_title.md) |
| related | [build](/crates/oxide-sim/src/deck/mod/build.md) |
| related | [build_pin_to_node_map](/crates/oxide-sim/src/deck/mod/build_pin_to_node_map.md) |
| related | [find_model_for_symbol](/crates/oxide-sim/src/deck/mod/find_model_for_symbol.md) |
| related | [new](/crates/oxide-sim/src/deck/mod/new.md) |
| related | [with_title](/crates/oxide-sim/src/deck/mod/with_title.md) |
| related | [build](/crates/oxide-sim/src/deck/mod/build.md) |
| related | [build_pin_to_node_map](/crates/oxide-sim/src/deck/mod/build_pin_to_node_map.md) |
| related | [find_model_for_symbol](/crates/oxide-sim/src/deck/mod/find_model_for_symbol.md) |
| related | [sanitize_node_name](/crates/oxide-sim/src/deck/mod/sanitize_node_name.md) |
| related | [sanitize_identifier](/crates/oxide-sim/src/deck/mod/sanitize_identifier.md) |
| related | [sanitize_spice_value](/crates/oxide-sim/src/deck/mod/sanitize_spice_value.md) |
| related | [build_rc_circuit_deck](/crates/oxide-sim/src/deck/mod/build_rc_circuit_deck.md) |
