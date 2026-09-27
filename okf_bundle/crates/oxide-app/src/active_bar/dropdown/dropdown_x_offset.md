---
okf_version: "0.2"
type: Function
title: dropdown_x_offset
description: Horizontal offset (in px) to align dropdown below a given button index.
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/dropdown_x_offset
language: rust
---

# dropdown_x_offset

Horizontal offset (in px) to align dropdown below a given button index.

## Signature

```rust
pub fn dropdown_x_offset(menu: ActiveBarMenu) -> f32
```

## Visibility

- `pub`

## Docstring

Horizontal offset (in px) to align dropdown below a given button index.

## Source
Lines 869–900 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| called_by | [active_bar_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/active_bar_menu_overlay.md) |
