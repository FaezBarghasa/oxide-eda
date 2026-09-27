---
okf_version: "0.2"
type: Module
title: sketch_mode
description: Sketch-mode tooling for the footprint editor.
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/mod
language: rust
---

# sketch_mode

Sketch-mode tooling for the footprint editor.

## Docstring

Sketch-mode tooling for the footprint editor.

Phase 5.5 + 6 of the v0.13 sketch-mode plan. The dispatcher in
[`crate::library::editor::footprint::sketch_dispatch`] consumes
[`SketchEdit`] values; the UI surface (Phase 6) routes
[`SketchModeMsg`] from iced through this module to the
dispatcher.

v0.13 ships the `messages` and `tools` modules; the inspector +
overlay + DOF render layer land in Phase 6 follow-ups.
