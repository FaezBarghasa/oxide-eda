---
okf_version: "0.2"
type: Function
title: fp_parse_optional_number
description: "Read an unbounded optional-number field (`hole_rotation_deg`)."
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number
language: rust
---

# fp_parse_optional_number

Read an unbounded optional-number field (`hole_rotation_deg`).

## Signature

```rust
pub(super) fn fp_parse_optional_number(value: &str) -> OptionalNumberEdit
```

## Visibility

- `pub(super)`

## Docstring

Read an unbounded optional-number field (`hole_rotation_deg`).

## Source
Lines 44–56 in `crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numeric_input](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.md) |
| called_by | [handle_fp_editor_set_next_pad_hole_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_rotation.md) |
| called_by | [handle_dock_sch_library_message](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message.md) |
| called_by | [fp_parse_optional_number_in](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in.md) |
| called_by | [non_finite_numbers_are_unreadable](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/non_finite_numbers_are_unreadable.md) |
