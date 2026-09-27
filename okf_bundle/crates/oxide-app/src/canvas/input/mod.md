---
okf_version: "0.2"
type: Module
title: input
description: "Input handling for the schematic canvas, split by concern:"
resource: crates/oxide-app/src/canvas/input/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/mod
language: rust
---

# input

Input handling for the schematic canvas, split by concern:

## Docstring

Input handling for the schematic canvas, split by concern:
camera (zoom/fit), pointer (buttons + motion), keys (keyboard).

Each file adds `impl CanvasSlot` methods that
`canvas::Program::update` dispatches to in the original event order.
