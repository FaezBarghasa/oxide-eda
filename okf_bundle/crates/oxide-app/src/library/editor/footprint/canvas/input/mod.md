---
okf_version: "0.2"
type: Module
title: input
description: "Canvas input handling — the `Program::update` body decomposed by"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/mod
language: rust
---

# input

Canvas input handling — the `Program::update` body decomposed by

## Docstring

Canvas input handling — the `Program::update` body decomposed by
concern into `impl FootprintCanvas` methods. The trait `update`
(in the parent `canvas` module) stays a thin dispatcher that calls
these in the original order; behaviour is byte-identical.

- [`camera`] — first-draw fit, Fit-to-Window, wheel zoom, pan.
- [`pointer`] — button-press / release / cursor-move dispatchers +
the shared classification helpers (snap, drag ticks, hover tail).
- [`tools`] — per-tool left-press gesture arms.
- [`release`] — left-release commit arms (split from `tools` for
the file-size cap).
- [`keys`] — modifier tracking + keyboard handling.
