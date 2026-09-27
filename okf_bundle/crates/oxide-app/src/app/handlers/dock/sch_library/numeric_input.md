---
okf_version: "0.2"
type: Module
title: numeric_input
description: "Properties-panel numeric edits that store into an `Option<f64>`."
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input
language: rust
---

# numeric_input

Properties-panel numeric edits that store into an `Option<f64>`.

## Docstring

Properties-panel numeric edits that store into an `Option<f64>`.

GH #599 — the pad form's `corner_radius_pct` and `hole_rotation_deg`
rows used to commit with `parse::<f64>().ok().filter(range)`, which
maps three different user intentions onto one `None`:

* the field was emptied      — "clear this value"
* the text is not a number   — "I am mid-edit / I mistyped"
* the number is out of range — "I meant a value near the limit"

That single `None` was then written straight over whatever was
stored, so a pad at 25 % corner radius edited to `60` lost the 25.
`corner_radius_pct` is a persisted UI mirror rather than geometry
(`oxide-library/src/primitive/footprint/pad.rs` documents that), and
it is serialised with `skip_serializing_if = "Option::is_none"`, so
clearing it drops the key from the saved `.snxfpt` outright — the
value no longer survives a shape switch and back.

These helpers keep the three intentions apart and report the two the
user did not ask for.

## Relationships

| Type | Target |
|------|--------|
| related | [OptionalNumberEdit](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/OptionalNumberEdit.md) |
| related | [fp_parse_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number.md) |
| related | [fp_parse_optional_number_in](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in.md) |
| related | [fp_resolve_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_resolve_optional_number.md) |
| related | [resolve](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/resolve.md) |
| related | [empty_field_clears_the_stored_value](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/empty_field_clears_the_stored_value.md) |
| related | [in_range_number_is_stored_as_typed](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/in_range_number_is_stored_as_typed.md) |
| related | [out_of_range_number_clamps_instead_of_clearing](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/out_of_range_number_clamps_instead_of_clearing.md) |
| related | [unreadable_text_refuses_the_write](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/unreadable_text_refuses_the_write.md) |
| related | [non_finite_numbers_are_unreadable](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/non_finite_numbers_are_unreadable.md) |
| related | [unbounded_field_keeps_negative_and_large_values](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/unbounded_field_keeps_negative_and_large_values.md) |
