---
okf_version: "0.2"
type: Function
title: grid_manager_btn
description: Shared button factory for the Grid / Guide Manager footers.
resource: crates/oxide-app/src/panels/footprint_editor_properties/managers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/managers/grid_manager_btn
language: rust
---

# grid_manager_btn

Shared button factory for the Grid / Guide Manager footers.

## Signature

```rust
pub(super) fn grid_manager_btn(
    label: &'static str,
    on_press: Option<PanelMsg>,
    primary: Color,
    _border_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Shared button factory for the Grid / Guide Manager footers.
Uses iced's built-in `button::primary` (accent-filled) so the
chrome matches the "+ Add Filter" call-to-action button in the
Custom Selection Filters section above.

## Source
Lines 212–225 in `crates/oxide-app/src/panels/footprint_editor_properties/managers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [managers](/crates/oxide-app/src/panels/footprint_editor_properties/managers.md) |
