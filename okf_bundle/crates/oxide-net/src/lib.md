---
okf_version: "0.2"
type: Module
title: lib
description: "Schematic connectivity → the authoritative [`Netlist`]."
resource: crates/oxide-net/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:35:55Z"
concept_id: crates/oxide-net/src/lib
language: rust
---

# lib

Schematic connectivity → the authoritative [`Netlist`].

## Docstring

Schematic connectivity → the authoritative [`Netlist`].

One place derives electrical nets from a `SchematicSheet` (union-find
over wires + junctions, named by labels, terminated at component pins).
ERC, the net-flood UI, the ratsnest, PCB net assignment, and the netlist
exporter are all meant to read this instead of hand-rolling their own
union-find (ADR-0001 A3.1). The union-find primitive itself lives in
[`uf`], shared with `oxide-erc`.

[`Netlist`]: oxide_types::net::Netlist
