---
okf_version: "0.2"
type: Function
title: fp_editor_set_snap_grid_step
description: "v0.18.9 — Properties-panel \"Grid step\" numeric input. Parses"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_snap_grid_step_1
language: rust
---

# fp_editor_set_snap_grid_step

v0.18.9 — Properties-panel "Grid step" numeric input. Parses

## Signature

```rust
pub(crate) fn fp_editor_set_snap_grid_step(&mut self, value: &str) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

v0.18.9 — Properties-panel "Grid step" numeric input. Parses
the user's text; on a clean positive parse writes
`state.snap_options.grid_step_mm`. Invalid / empty / non-
positive strings no-op so partial keystrokes don't snap to
zero (which would crash the canvas's grid math).

## Source
Lines 226–247 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
