---
okf_version: "0.2"
type: Module
title: input
description: "Canvas input handling — the `Program::update` body decomposed by"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/mod
language: rust
---

# input

Canvas input handling — the `Program::update` body decomposed by

## Docstring

Canvas input handling — the `Program::update` body decomposed by
concern into `impl SymbolCanvas` methods. The trait `update` (in
the parent `canvas` module) stays a thin dispatcher that routes
each event kind to one of these in the original arm order;
behaviour is byte-identical.

- [`tools`] — left-press per-tool gesture arms (Select hit-test,
pin / rectangle / line / circle / arc / text placement).
- [`pointer`] — pan start/stop, cursor-move drag + rubber-band
tracking, and left-release commit.
- [`camera`] — scroll-wheel zoom (cursor-anchored).
- [`keys`] — keyboard handling (Escape cancel, Delete, Home,
rotate, undo/redo, select-all).
