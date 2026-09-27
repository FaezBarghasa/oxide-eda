---
okf_version: "0.2"
type: Module
title: sch_library
description: Dock library/editor panel message dispatcher. Routes the
resource: crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/mod
language: rust
---

# sch_library

Dock library/editor panel message dispatcher. Routes the

## Docstring

Dock library/editor panel message dispatcher. Routes the
`DockMessage::Panel(PanelMsg::…)` variants that mutate the active
`.snxsym` / `.snxfpt` container — SCH Library, Symbol editor,
Footprint Library, and Footprint editor panels.

`handle_dock_sch_library_message` is a thin router; every arm with
real logic delegates to a `handle_*` method living in the matching
concern module:
- [`symbol`] — `SchLibrary*` / `SymEditor*` mutators.
- [`footprint::pad`] — active-editor accessors + pad-defaults setters.
- [`footprint::shape`] — pour / keepout / cutout / snap / array.
- [`footprint::library`] — Footprint Library panel (envelope CRUD).
- [`footprint::props`] — footprint component-level properties.
- [`footprint::grid`] — grid / guide managers + snap sub-tab / mode.
- [`footprint::silk`] — selected silk-graphic edits.
- [`footprint::sketch`] — sketch-entity jumps + parameter forwards.

Mutations mark the tab dirty and clear the canvas cache; the actual
save to disk happens through the existing Save flow
(`save_primitive_tab_at`) so the panel never writes the file
directly — keeps the dirty / save semantics consistent with every
other in-tab mutation.

Split out of the former `sch_library.rs` god-file (ADR-0001 #163);
pure code motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [active_footprint_editor_path](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/active_footprint_editor_path.md) |
| related | [handle_dock_sch_library_message](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message.md) |
| related | [active_footprint_editor_path](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/active_footprint_editor_path.md) |
| related | [handle_dock_sch_library_message](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message.md) |
| related | [fp_parse_optional_mm](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/fp_parse_optional_mm.md) |
| related | [apply_graphic_field](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/apply_graphic_field.md) |
| related | [arc_degree_edits_stay_in_range_and_preserve_sweep](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/arc_degree_edits_stay_in_range_and_preserve_sweep.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
