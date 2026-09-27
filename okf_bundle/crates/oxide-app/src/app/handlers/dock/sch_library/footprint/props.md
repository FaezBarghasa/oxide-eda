---
okf_version: "0.2"
type: Module
title: props
description: Footprint-editor component-level property handlers — the methods
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props
language: rust
---

# props

Footprint-editor component-level property handlers — the methods

## Docstring

Footprint-editor component-level property handlers — the methods
behind the `FpEditor*` dock-panel messages that edit the active
`.snxfpt` footprint's own metadata (name, description, default
designator, component type, height, sketch role, auto-fit
courtyard, selection filter). The dispatcher in `mod.rs` routes
these panel messages here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_fp_editor_toggle_auto_fit_courtyard](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_toggle_auto_fit_courtyard.md) |
| related | [handle_fp_editor_set_role](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_role.md) |
| related | [handle_fp_editor_set_footprint_description](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_description.md) |
| related | [handle_fp_editor_set_footprint_default_designator](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_default_designator.md) |
| related | [handle_fp_editor_set_footprint_component_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_component_type.md) |
| related | [handle_fp_editor_set_footprint_height](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_height.md) |
| related | [handle_fp_editor_set_footprint_name](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_name.md) |
| related | [handle_fp_editor_toggle_selection_filter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_toggle_selection_filter.md) |
| related | [handle_fp_editor_toggle_auto_fit_courtyard](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_toggle_auto_fit_courtyard.md) |
| related | [handle_fp_editor_set_role](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_role.md) |
| related | [handle_fp_editor_set_footprint_description](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_description.md) |
| related | [handle_fp_editor_set_footprint_default_designator](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_default_designator.md) |
| related | [handle_fp_editor_set_footprint_component_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_component_type.md) |
| related | [handle_fp_editor_set_footprint_height](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_height.md) |
| related | [handle_fp_editor_set_footprint_name](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_name.md) |
| related | [handle_fp_editor_toggle_selection_filter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_toggle_selection_filter.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
