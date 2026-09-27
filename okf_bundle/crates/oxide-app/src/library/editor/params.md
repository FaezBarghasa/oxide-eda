---
okf_version: "0.2"
type: Module
title: params
description: "Parameters tab — template-validated parameter form, retargeted at"
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params
language: rust
---

# params

Parameters tab — template-validated parameter form, retargeted at

## Docstring

Parameters tab — template-validated parameter form, retargeted at
`state.row.parameters` (DBLib model).

Renders three groups against the active class template:
- **Required:** every `ParameterTemplate.required_params` slot, with
a "✗ missing" amber tag when no value is currently bound.
- **Optional:** every `ParameterTemplate.optional_params` slot.
- **Custom:** parameters on the row that aren't in the template.
Custom rows carry an inline `[×]` remove button.

When no template resolves the view falls back to a "no template —
populate with custom parameters" surface that only renders the
custom rows + the add-new control.

Numeric / measurement edits go through a per-row `String` buffer on
`ComponentPreviewState.params_edit_buf`, following the
`reference_erasable_numeric_input` pattern.

## Relationships

| Type | Target |
|------|--------|
| related | [resolve_template](/crates/oxide-app/src/library/editor/params/resolve_template.md) |
| related | [view](/crates/oxide-app/src/library/editor/params/view.md) |
| related | [template_row](/crates/oxide-app/src/library/editor/params/template_row.md) |
| related | [custom_row](/crates/oxide-app/src/library/editor/params/custom_row.md) |
| related | [slot_input](/crates/oxide-app/src/library/editor/params/slot_input.md) |
| related | [display_param](/crates/oxide-app/src/library/editor/params/display_param.md) |
| related | [add_custom_row](/crates/oxide-app/src/library/editor/params/add_custom_row.md) |
| related | [missing_required_for_test](/crates/oxide-app/src/library/editor/params/missing_required_for_test.md) |
| related | [empty_state_no_template_no_params](/crates/oxide-app/src/library/editor/params/empty_state_no_template_no_params.md) |
| related | [template_with_required_and_optional_fully_populated](/crates/oxide-app/src/library/editor/params/template_with_required_and_optional_fully_populated.md) |
| related | [template_with_missing_required_flag_visible](/crates/oxide-app/src/library/editor/params/template_with_missing_required_flag_visible.md) |
| related | [measurement_parse_round_trip_via_buffer](/crates/oxide-app/src/library/editor/params/measurement_parse_round_trip_via_buffer.md) |
| related | [bad_parse_keeps_buffer_dirty_and_skips_commit](/crates/oxide-app/src/library/editor/params/bad_parse_keeps_buffer_dirty_and_skips_commit.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
