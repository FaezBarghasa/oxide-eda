---
okf_version: "0.2"
type: Function
title: view_dropdown
description: Render the dropdown menu for the given Active Bar button.
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/view_dropdown
language: rust
---

# view_dropdown

Render the dropdown menu for the given Active Bar button.

## Signature

```rust
pub fn view_dropdown(
    menu: ActiveBarMenu,
    tokens: &'a ThemeTokens,
    filters: &std::collections::HashSet<SelectionFilter>,
    custom_presets: &[CustomFilterPreset],
    tid: ThemeId,
    has_selection: bool,
    has_net_colors: bool,
) -> Element<'a, ActiveBarMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the dropdown menu for the given Active Bar button.

## Source
Lines 25–51 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| calls | [dropdown_entries](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries.md) |
| calls | [dropdown_min_width](/crates/oxide-app/src/active_bar/dropdown/dropdown_min_width.md) |
| called_by | [active_bar_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/active_bar_menu_overlay.md) |
