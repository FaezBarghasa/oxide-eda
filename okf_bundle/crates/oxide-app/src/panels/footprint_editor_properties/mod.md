---
okf_version: "0.2"
type: Module
title: footprint_editor_properties
description: Properties panel body for the Footprint editor (HI-22 / MD-20).
resource: crates/oxide-app/src/panels/footprint_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/mod
language: rust
---

# footprint_editor_properties

Properties panel body for the Footprint editor (HI-22 / MD-20).

## Docstring

Properties panel body for the Footprint editor (HI-22 / MD-20).

Extracted from `panels/mod.rs`. Pure view code — zero behaviour change
from the move. Switches between Pads / Sketch / 3D View contexts and
renders the v0.18.13 Library Options sections (Snap Options / Grid /
Guide / Other) plus the v0.16.4 role sub-forms (Pour / Keepout /
Cutout) and the v0.16.3 Pad placement defaults form.

## Relationships

| Type | Target |
|------|--------|
| related | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
| related | [render_fp_settings_and_hint](/crates/oxide-app/src/panels/footprint_editor_properties/mod/render_fp_settings_and_hint.md) |
| related | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| related | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| related | [props_kv_row](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_kv_row.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
