---
okf_version: "0.2"
type: Function
title: color_field
description: "Render a colour-selection field. See [`ColorFieldProps`]."
resource: crates/oxide-app/src/panels/color_field.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/color_field/color_field
language: rust
---

# color_field

Render a colour-selection field. See [`ColorFieldProps`].

## Signature

```rust
pub fn color_field(props: ColorFieldProps<'a, M>) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: Clone + 'static`

## Visibility

- `pub`

## Docstring

Render a colour-selection field. See [`ColorFieldProps`].

`M: 'static` because the `on_pick` callback (and the `iced_aw`
submit closure it feeds) must outlive the widget; panel messages are
all `'static`, so this holds everywhere.

## Source
Lines 79–262 in `crates/oxide-app/src/panels/color_field.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [color_field](/crates/oxide-app/src/panels/color_field.md) |
| calls | [color_to_rgba](/crates/oxide-app/src/panels/color_field/color_to_rgba.md) |
| called_by | [child_sheet_color_row](/crates/oxide-app/src/panels/element_properties/child_sheet/child_sheet_color_row.md) |
| called_by | [graphic_fill_field](/crates/oxide-app/src/panels/symbol_editor_properties/graphic/graphic_fill_field.md) |
| called_by | [local_color_field](/crates/oxide-app/src/panels/symbol_editor_properties/symbol/local_color_field.md) |
