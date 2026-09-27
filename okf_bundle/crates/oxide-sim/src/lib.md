---
okf_version: "0.2"
type: Module
title: lib
description: "`oxide-sim` — Circuit simulation engine and PSpice deck generator for Oxide EDA."
resource: crates/oxide-sim/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:31:07Z"
concept_id: crates/oxide-sim/src/lib
language: rust
---

# lib

`oxide-sim` — Circuit simulation engine and PSpice deck generator for Oxide EDA.

## Docstring

`oxide-sim` — Circuit simulation engine and PSpice deck generator for Oxide EDA.

Provides the [`Simulator`] port trait with concrete adapters:
- [`NgSpiceSimulator`]: Local/built-in ngspice running in PSpice mode (`set ngbehavior=ps`).
- [`PSpiceCliSimulator`]: External Cadence OrCAD PSpice CLI batch runner.

Also provides the [`PSpiceDeckBuilder`] for translating schematic connectivity
and component models into valid PSpice simulation decks, plus result parsers
and mathematical analysis helpers.
