---
okf_version: "0.2"
type: Function
title: view_library_row_properties
description: F15 — Library Browser row detail in the Properties panel. Shows
resource: crates/oxide-app/src/panels/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/library/view_library_row_properties
language: rust
---

# view_library_row_properties

F15 — Library Browser row detail in the Properties panel. Shows

## Signature

```rust
pub fn view_library_row_properties(
    d: &'a LibraryRowDetail,
    muted: iced::Color,
    primary: iced::Color,
    border_c: iced::Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

F15 — Library Browser row detail in the Properties panel. Shows
the row's identifier line + Symbol / Footprint binding status with
Pick buttons. Mirrors what the Library Browser's inline preview
pane used to render; surfacing here means the user gets the row's
detail in the canonical "selected thing" panel, freeing horizontal
space inside the browser tab for the grid.

## Source
Lines 514–592 in `crates/oxide-app/src/panels/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/panels/library.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
