---
okf_version: "0.2"
type: Module
title: form
description: "Pad-form types, message-helper macro, widget primitives, and the"
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod
language: rust
---

# form

Pad-form types, message-helper macro, widget primitives, and the

## Docstring

Pad-form types, message-helper macro, widget primitives, and the
three Properties-panel render functions for Pads-mode (Properties
/ Pad Stack / Pad Features).

Cross-module surface:
- `PadEditTarget` / `PadFormValues` are exported `pub(super)` so
`pad_table`, `pad_stack_preview`, and the role sub-forms can
share the same form snapshot + edit-routing semantics.
- The `pad_msg_fns!` macro stamps out ~30 near-identical message-
builder helpers; without it each builder cost ~7 lines of
boilerplate. The macro is declared file-local but the emitted
`pub(super) fn` are reachable from `pad_table`.
- `pad_input_row` / `pad_pick_row` / `pad_check_row` are the row-
chrome primitives; `subforms` and `pad_table` import them.

## Relationships

| Type | Target |
|------|--------|
| related | [PadEditTarget](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/PadEditTarget.md) |
| related | [PadFormValues](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/PadFormValues.md) |
| related | [from_next_pad](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/from_next_pad.md) |
| related | [from_selected_pad](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/from_selected_pad.md) |
| related | [from_next_pad](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/from_next_pad.md) |
| related | [from_selected_pad](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/from_selected_pad.md) |
| related | [pad_input_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_input_row.md) |
| related | [pad_pick_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_pick_row.md) |
| related | [pad_check_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_check_row.md) |
| related | [render_pad_form_properties](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_properties.md) |
| related | [render_pad_form_pad_features](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_pad_features.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
