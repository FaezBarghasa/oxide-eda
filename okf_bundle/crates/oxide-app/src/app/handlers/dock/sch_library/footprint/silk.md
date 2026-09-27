---
okf_version: "0.2"
type: Module
title: silk
description: Footprint-editor silkscreen graphic handlers — the methods behind
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk
language: rust
---

# silk

Footprint-editor silkscreen graphic handlers — the methods behind

## Docstring

Footprint-editor silkscreen graphic handlers — the methods behind
the `FpEditor*` dock-panel messages that edit the selected
silk-front graphic (Line endpoints, Text position / size / content,
stroke width, filled toggle, delete) on the active `.snxfpt`
editor. Owns the `SilkLineEndpoint` / `SilkTextField` selector enums
the endpoint / field setters take. The dispatcher in `mod.rs` routes
these panel messages here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [SilkLineEndpoint](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/SilkLineEndpoint.md) |
| related | [SilkTextField](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/SilkTextField.md) |
| related | [fp_editor_set_silk_line_endpoint](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/fp_editor_set_silk_line_endpoint.md) |
| related | [fp_editor_set_silk_text_field](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/fp_editor_set_silk_text_field.md) |
| related | [handle_fp_editor_set_silk_stroke_width](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_set_silk_stroke_width.md) |
| related | [handle_fp_editor_toggle_silk_filled](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_toggle_silk_filled.md) |
| related | [handle_fp_editor_set_silk_text](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_set_silk_text.md) |
| related | [handle_fp_editor_delete_selected_silk](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_delete_selected_silk.md) |
| related | [fp_editor_set_silk_line_endpoint](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/fp_editor_set_silk_line_endpoint.md) |
| related | [fp_editor_set_silk_text_field](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/fp_editor_set_silk_text_field.md) |
| related | [handle_fp_editor_set_silk_stroke_width](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_set_silk_stroke_width.md) |
| related | [handle_fp_editor_toggle_silk_filled](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_toggle_silk_filled.md) |
| related | [handle_fp_editor_set_silk_text](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_set_silk_text.md) |
| related | [handle_fp_editor_delete_selected_silk](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_delete_selected_silk.md) |
