---
okf_version: "0.2"
type: Function
title: shape_icon_handle
resource: crates/oxide-app/src/panels/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/widgets/shape_icon_handle
language: rust
---

# shape_icon_handle

## Signature

```rust
pub fn shape_icon_handle(
    elem_type: &str,
    theme: oxide_types::theme::ThemeId,
) -> Option<svg::Handle>
```

## Visibility

- `pub`

## Source
Lines 23–35 in `crates/oxide-app/src/panels/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/panels/widgets.md) |
| called_by | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
