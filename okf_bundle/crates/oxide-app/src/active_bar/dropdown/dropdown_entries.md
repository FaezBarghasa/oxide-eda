---
okf_version: "0.2"
type: Function
title: dropdown_entries
description: "Route each `ActiveBarMenu` to its entries. Uniform menus resolve"
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/dropdown_entries
language: rust
---

# dropdown_entries

Route each `ActiveBarMenu` to its entries. Uniform menus resolve

## Signature

```rust
fn dropdown_entries(
    menu: ActiveBarMenu,
    tokens: &ThemeTokens,
    filters: &std::collections::HashSet<SelectionFilter>,
    custom_presets: &[CustomFilterPreset],
    tid: ThemeId,
    has_selection: bool,
    has_net_colors: bool,
) -> Vec<DropdownEntry<ActiveBarMsg>>
```

## Docstring

Route each `ActiveBarMenu` to its entries. Uniform menus resolve
through the `EntrySpec` data table + `render` below; the two
irregular menus (Filter chip grid and NetColor swatches) use the
widget's `Custom` escape hatch and keep their own builder.

## Source
Lines 81–107 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| calls | [render](/crates/oxide-app/src/active_bar/dropdown/render.md) |
| calls | [net_color_entries](/crates/oxide-app/src/active_bar/dropdown/net_color_entries.md) |
| called_by | [dropdown_entries_match_pre_refactor_golden](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries_match_pre_refactor_golden.md) |
| called_by | [view_dropdown](/crates/oxide-app/src/active_bar/dropdown/view_dropdown.md) |
