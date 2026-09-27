---
okf_version: "0.2"
type: Module
title: inspector
description: v0.13.1 Phase 6.5 — sketch inspector (lite).
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector
language: rust
---

# inspector

v0.13.1 Phase 6.5 — sketch inspector (lite).

## Docstring

v0.13.1 Phase 6.5 — sketch inspector (lite).

Three sections, displayed as a horizontal strip below the
footprint toolbar when [`EditorMode::Sketch`] is active:

1. **DOF readout** — `state.len()` / constraint count / rank
(free DoF) + last solve elapsed_ms + auto-pause status.
2. **Parameter table** — list of file-local parameters; each row
is editable in place. New rows are appended via the `+ Add`
button.
3. **Solve warnings** — pad-bake warnings surfaced from the most
recent solve (Castellated bakes as Tht, Chamfered → RoundRect,
PasteAperturePattern Grid/Custom deferred, etc.).

Selection editing (per-entity coords, attached constraints with
delete buttons, auto-Coincident toggle) is deferred to v0.13.2 —
requires per-entity hit-testing in the canvas which lives in the
same patch as Task 6.3's tool palette.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view.md) |
| related | [view_tool_palette](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_tool_palette.md) |
| related | [view_constraint_submenu](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_constraint_submenu.md) |
| related | [view_dof](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_dof.md) |
| related | [view_params](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_params.md) |
| related | [view_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_warnings.md) |
| related | [view_role](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_role.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
