---
okf_version: "0.2"
type: Module
title: pin_map
description: Pin-map override edits for a Component Preview row.
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map
language: rust
---

# pin_map

Pin-map override edits for a Component Preview row.

## Docstring

Pin-map override edits for a Component Preview row.

The pin map defaults to a 1:1 symbol-pin → footprint-pad mapping by
number; this module owns the inline override editor that lets a user
pin specific pads, plus the toolbar actions that clear or auto-match
the overrides.

## Relationships

| Type | Target |
|------|--------|
| related | [clear_overrides](/crates/oxide-app/src/library/component_preview/updates/pin_map/clear_overrides.md) |
| related | [warn_auto_match_by_name](/crates/oxide-app/src/library/component_preview/updates/pin_map/warn_auto_match_by_name.md) |
| related | [open_override_edit](/crates/oxide-app/src/library/component_preview/updates/pin_map/open_override_edit.md) |
| related | [set_override_buf](/crates/oxide-app/src/library/component_preview/updates/pin_map/set_override_buf.md) |
| related | [add_override](/crates/oxide-app/src/library/component_preview/updates/pin_map/add_override.md) |
| related | [cancel_override_edit](/crates/oxide-app/src/library/component_preview/updates/pin_map/cancel_override_edit.md) |
| related | [remove_override](/crates/oxide-app/src/library/component_preview/updates/pin_map/remove_override.md) |
