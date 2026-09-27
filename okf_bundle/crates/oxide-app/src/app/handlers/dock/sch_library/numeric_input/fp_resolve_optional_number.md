---
okf_version: "0.2"
type: Function
title: fp_resolve_optional_number
description: "Resolve a parsed edit into what to store, reporting anything the"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_resolve_optional_number
language: rust
---

# fp_resolve_optional_number

Resolve a parsed edit into what to store, reporting anything the

## Signature

```rust
pub(super) fn fp_resolve_optional_number(
    field: &'static str,
    value: &str,
    edit: OptionalNumberEdit,
) -> Option<Option<f64>>
```

## Visibility

- `pub(super)`

## Docstring

Resolve a parsed edit into what to store, reporting anything the
user asked for and did not get.

`None` means **do not write**: the text is not a number, so the
caller must leave the stored value alone. `Some(slot)` is the new
contents of the `Option<f64>`.

Levels follow the two outcomes: a clamp is a degraded path that
still stored something (`warn!`), a refused edit withheld the user's
change outright (`error!`). Neither may be `debug!` — the default
filter is `LevelFilter::Info` (`crate::diagnostics`), so a `debug!`
record never reaches the Messages panel.

## Source
Lines 84–112 in `crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numeric_input](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.md) |
| called_by | [fp_editor_set_next_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_corner_radius_pct.md) |
| called_by | [fp_editor_set_selected_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_corner_radius_pct.md) |
| called_by | [handle_fp_editor_set_next_pad_hole_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_rotation.md) |
| called_by | [handle_dock_sch_library_message](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message.md) |
| called_by | [resolve](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/resolve.md) |
