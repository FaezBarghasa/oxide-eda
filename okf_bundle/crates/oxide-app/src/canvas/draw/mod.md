---
okf_version: "0.2"
type: Module
title: draw
description: "Rendering for the schematic canvas, split by draw layer (bottom-to-top):"
resource: crates/oxide-app/src/canvas/draw/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/draw/mod
language: rust
---

# draw

Rendering for the schematic canvas, split by draw layer (bottom-to-top):

## Docstring

Rendering for the schematic canvas, split by draw layer (bottom-to-top):
background, scene (content + auto-focus dim + selection), then the Layer-4
overlay broken into previews, ghosts, drag guides, and cursor HUD.

Each file adds `impl CanvasSlot` methods; `canvas::Program::draw`
assembles them in the original z-order.
