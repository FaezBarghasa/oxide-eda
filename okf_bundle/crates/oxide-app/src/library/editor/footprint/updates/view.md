---
okf_version: "0.2"
type: Module
title: view
description: Footprint editor — view update logic.
resource: crates/oxide-app/src/library/editor/footprint/updates/view.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/view
language: rust
---

# view

Footprint editor — view update logic.

## Docstring

Footprint editor — view update logic.

Split out of `apply_footprint_primitive_edit` per ADR-0001 D1/D2.
`apply` is a thin router; each `FootprintEditorMsg` variant delegates
to one named per-action fn below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/view/apply.md) |
| related | [toggle_layer](/crates/oxide-app/src/library/editor/footprint/updates/view/toggle_layer.md) |
| related | [toggle_auto_fit](/crates/oxide-app/src/library/editor/footprint/updates/view/toggle_auto_fit.md) |
| related | [set_mode](/crates/oxide-app/src/library/editor/footprint/updates/view/set_mode.md) |
| related | [toggle_placement_pause](/crates/oxide-app/src/library/editor/footprint/updates/view/toggle_placement_pause.md) |
| related | [fit_consumed](/crates/oxide-app/src/library/editor/footprint/updates/view/fit_consumed.md) |
| related | [set_pads_tool](/crates/oxide-app/src/library/editor/footprint/updates/view/set_pads_tool.md) |
| related | [tool_escape](/crates/oxide-app/src/library/editor/footprint/updates/view/tool_escape.md) |
| related | [align_pads_action](/crates/oxide-app/src/library/editor/footprint/updates/view/align_pads_action.md) |
| related | [set_name](/crates/oxide-app/src/library/editor/footprint/updates/view/set_name.md) |
| related | [recompute_courtyard_outline](/crates/oxide-app/src/library/editor/footprint/updates/view/recompute_courtyard_outline.md) |
