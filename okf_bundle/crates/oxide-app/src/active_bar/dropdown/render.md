---
okf_version: "0.2"
type: Function
title: render
description: "Render a uniform per-menu `EntrySpec` table into `DropdownEntry`"
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/render
language: rust
---

# render

Render a uniform per-menu `EntrySpec` table into `DropdownEntry`

## Signature

```rust
fn render(
    specs: &'static [EntrySpec],
    tid: ThemeId,
    sel: bool,
    nc: bool,
) -> Vec<DropdownEntry<ActiveBarMsg>>
```

## Docstring

Render a uniform per-menu `EntrySpec` table into `DropdownEntry`
rows, resolving each row's icon for the active theme and folding in
the same `dd_item` enable/disable gating every row used before the
table refactor.

## Source
Lines 503–520 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| calls | [dd_item](/crates/oxide-app/src/active_bar/dropdown/dd_item.md) |
| called_by | [align_and_shapes_have_expected_shape](/crates/oxide-app/src/active_bar/dropdown/align_and_shapes_have_expected_shape.md) |
| called_by | [disabled_state_flips_but_labels_are_stable](/crates/oxide-app/src/active_bar/dropdown/disabled_state_flips_but_labels_are_stable.md) |
| called_by | [dropdown_entries](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries.md) |
| called_by | [select_mode_toggle_sits_after_the_separator](/crates/oxide-app/src/active_bar/dropdown/select_mode_toggle_sits_after_the_separator.md) |
| called_by | [wiring_menu_is_four_ungated_items](/crates/oxide-app/src/active_bar/dropdown/wiring_menu_is_four_ungated_items.md) |
