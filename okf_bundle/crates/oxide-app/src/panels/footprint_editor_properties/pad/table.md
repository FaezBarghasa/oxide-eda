---
okf_version: "0.2"
type: Module
title: table
description: Pad-stack table cells + the per-pad copper row.
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table
language: rust
---

# table

Pad-stack table cells + the per-pad copper row.

## Docstring

Pad-stack table cells + the per-pad copper row.

These helpers render the pad-properties table chrome (header, body
rows, individual data cells) — used by `pad_form::render_pad_form_pad_stack`
to compose the COPPER / HOLE / PASTE / SOLDER rows.

## Relationships

| Type | Target |
|------|--------|
| related | [pad_table_header](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_header.md) |
| related | [pad_table_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_row.md) |
| related | [pad_table_input_cell](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_input_cell.md) |
| related | [pad_table_picklist_cell](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_picklist_cell.md) |
| related | [pad_table_check_cell](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_check_cell.md) |
| related | [pad_table_disabled_cell](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_disabled_cell.md) |
| related | [pad_table_static_cell](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_static_cell.md) |
| related | [pad_copper_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_copper_row.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
