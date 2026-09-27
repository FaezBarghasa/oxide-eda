---
okf_version: "0.2"
type: Module
title: camera
description: "Camera input handling — first-draw fit, one-shot Fit-to-Window,"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/camera
language: rust
---

# camera

Camera input handling — first-draw fit, one-shot Fit-to-Window,

## Docstring

Camera input handling — first-draw fit, one-shot Fit-to-Window,
scroll-wheel zoom (cursor-anchored), and middle/right-drag panning.

Extracted verbatim from the canvas `Program::update` god-function;
behaviour is byte-identical — same conditions, same coordinate
math, same `Action::publish` / capture sites.

## Relationships

| Type | Target |
|------|--------|
| related | [apply_initial_fit](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/apply_initial_fit.md) |
| related | [apply_pending_fit](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/apply_pending_fit.md) |
| related | [on_wheel_scrolled](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/on_wheel_scrolled.md) |
| related | [pan_on_cursor_moved](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/pan_on_cursor_moved.md) |
| related | [apply_initial_fit](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/apply_initial_fit.md) |
| related | [apply_pending_fit](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/apply_pending_fit.md) |
| related | [on_wheel_scrolled](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/on_wheel_scrolled.md) |
| related | [pan_on_cursor_moved](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera/pan_on_cursor_moved.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
