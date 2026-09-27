---
okf_version: "0.2"
type: Module
title: symbol_editor_properties
description: "Properties panel for the active `.snxsym` standalone editor tab (HI-22 / MD-20)."
resource: crates/oxide-app/src/panels/symbol_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/mod
language: rust
---

# symbol_editor_properties

Properties panel for the active `.snxsym` standalone editor tab (HI-22 / MD-20).

## Docstring

Properties panel for the active `.snxsym` standalone editor tab (HI-22 / MD-20).

Extracted from `panels/mod.rs`. Pure view code, zero behaviour change.
Mirrors Altium SchLib's right-dock Properties: pin selected → pin
properties (editable Designator / Name / Length, read-only
Electrical / Position / Orientation), graphic selected → per-shape
numeric fields, field selected → field properties, nothing selected
→ symbol-level defaults (Name / UUID / pin count) with Name editable.

## Relationships

| Type | Target |
|------|--------|
| related | [view_symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod/view_symbol_editor_properties.md) |
| related | [prop_row_static](/crates/oxide-app/src/panels/symbol_editor_properties/mod/prop_row_static.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
