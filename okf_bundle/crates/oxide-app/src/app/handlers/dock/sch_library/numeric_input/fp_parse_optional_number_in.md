---
okf_version: "0.2"
type: Function
title: fp_parse_optional_number_in
description: "Read a range-limited optional-number field (`corner_radius_pct`)."
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in
language: rust
---

# fp_parse_optional_number_in

Read a range-limited optional-number field (`corner_radius_pct`).

## Signature

```rust
pub(super) fn fp_parse_optional_number_in(value: &str, min: f64, max: f64) -> OptionalNumberEdit
```

## Visibility

- `pub(super)`

## Docstring

Read a range-limited optional-number field (`corner_radius_pct`).
Out-of-range clamps to the nearest bound instead of clearing.

## Source
Lines 60–70 in `crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numeric_input](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.md) |
| calls | [fp_parse_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number.md) |
| called_by | [fp_editor_set_next_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_corner_radius_pct.md) |
| called_by | [fp_editor_set_selected_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_corner_radius_pct.md) |
| called_by | [handle_dock_sch_library_message](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message.md) |
| called_by | [empty_field_clears_the_stored_value](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/empty_field_clears_the_stored_value.md) |
| called_by | [in_range_number_is_stored_as_typed](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/in_range_number_is_stored_as_typed.md) |
| called_by | [out_of_range_number_clamps_instead_of_clearing](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/out_of_range_number_clamps_instead_of_clearing.md) |
| called_by | [unreadable_text_refuses_the_write](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/unreadable_text_refuses_the_write.md) |
