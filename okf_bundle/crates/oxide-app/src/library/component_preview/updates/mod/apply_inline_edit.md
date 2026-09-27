---
okf_version: "0.2"
type: Function
title: apply_inline_edit
description: Apply an inline-edit message to a Component Preview state.
resource: crates/oxide-app/src/library/component_preview/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit
language: rust
---

# apply_inline_edit

Apply an inline-edit message to a Component Preview state.

## Signature

```rust
pub(crate) fn apply_inline_edit(state: &mut ComponentPreviewState, msg: EditorMsg)
```

## Visibility

- `pub(crate)`

## Docstring

Apply an inline-edit message to a Component Preview state.

Tab switching, save, and async-bounce variants are handled by the
dispatcher before reaching here; this is the reducer for in-place row
mutations across the datasheet, pin-map, supply, parameters, and
simulation field groups.

## Source
Lines 29–140 in `crates/oxide-app/src/library/component_preview/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/component_preview/updates/mod.md) |
| calls | [set_url](/crates/oxide-app/src/library/component_preview/updates/datasheet/set_url.md) |
| calls | [apply_upload_result](/crates/oxide-app/src/library/component_preview/updates/datasheet/apply_upload_result.md) |
| calls | [clear_overrides](/crates/oxide-app/src/library/component_preview/updates/pin_map/clear_overrides.md) |
| calls | [warn_auto_match_by_name](/crates/oxide-app/src/library/component_preview/updates/pin_map/warn_auto_match_by_name.md) |
| calls | [open_override_edit](/crates/oxide-app/src/library/component_preview/updates/pin_map/open_override_edit.md) |
| calls | [set_override_buf](/crates/oxide-app/src/library/component_preview/updates/pin_map/set_override_buf.md) |
| calls | [add_override](/crates/oxide-app/src/library/component_preview/updates/pin_map/add_override.md) |
| calls | [cancel_override_edit](/crates/oxide-app/src/library/component_preview/updates/pin_map/cancel_override_edit.md) |
| calls | [remove_override](/crates/oxide-app/src/library/component_preview/updates/pin_map/remove_override.md) |
| calls | [set_primary_manufacturer](/crates/oxide-app/src/library/component_preview/updates/supply/set_primary_manufacturer.md) |
| calls | [set_primary_mpn](/crates/oxide-app/src/library/component_preview/updates/supply/set_primary_mpn.md) |
| calls | [set_primary_status](/crates/oxide-app/src/library/component_preview/updates/supply/set_primary_status.md) |
| calls | [set_primary_notes](/crates/oxide-app/src/library/component_preview/updates/supply/set_primary_notes.md) |
| calls | [add_alternate](/crates/oxide-app/src/library/component_preview/updates/supply/add_alternate.md) |
| calls | [set_alternate_manufacturer](/crates/oxide-app/src/library/component_preview/updates/supply/set_alternate_manufacturer.md) |
| calls | [set_alternate_mpn](/crates/oxide-app/src/library/component_preview/updates/supply/set_alternate_mpn.md) |
| calls | [set_alternate_status](/crates/oxide-app/src/library/component_preview/updates/supply/set_alternate_status.md) |
| calls | [set_alternate_notes](/crates/oxide-app/src/library/component_preview/updates/supply/set_alternate_notes.md) |
| calls | [remove_alternate](/crates/oxide-app/src/library/component_preview/updates/supply/remove_alternate.md) |
| calls | [add_listing](/crates/oxide-app/src/library/component_preview/updates/supply/add_listing.md) |
| calls | [set_listing_distributor](/crates/oxide-app/src/library/component_preview/updates/supply/set_listing_distributor.md) |
| calls | [set_listing_sku](/crates/oxide-app/src/library/component_preview/updates/supply/set_listing_sku.md) |
| calls | [set_listing_url](/crates/oxide-app/src/library/component_preview/updates/supply/set_listing_url.md) |
| calls | [remove_listing](/crates/oxide-app/src/library/component_preview/updates/supply/remove_listing.md) |
| calls | [set_text](/crates/oxide-app/src/library/component_preview/updates/parameters/set_text.md) |
| calls | [set_number_buf](/crates/oxide-app/src/library/component_preview/updates/parameters/set_number_buf.md) |
| calls | [commit_number](/crates/oxide-app/src/library/component_preview/updates/parameters/commit_number.md) |
| calls | [set_measurement_buf](/crates/oxide-app/src/library/component_preview/updates/parameters/set_measurement_buf.md) |
| calls | [commit_measurement](/crates/oxide-app/src/library/component_preview/updates/parameters/commit_measurement.md) |
| calls | [set_bool](/crates/oxide-app/src/library/component_preview/updates/parameters/set_bool.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [add_custom](/crates/oxide-app/src/library/component_preview/updates/parameters/add_custom.md) |
| calls | [set_enabled](/crates/oxide-app/src/library/component_preview/updates/sim/set_enabled.md) |
| calls | [set_kind](/crates/oxide-app/src/library/component_preview/updates/sim/set_kind.md) |
| calls | [apply_body_action](/crates/oxide-app/src/library/component_preview/updates/sim/apply_body_action.md) |
| calls | [set_pin_node](/crates/oxide-app/src/library/component_preview/updates/sim/set_pin_node.md) |
| called_by | [handle_editor_event](/crates/oxide-app/src/app/dispatch/library/component_preview/handle_editor_event.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/sim/mod/apply.md) |
