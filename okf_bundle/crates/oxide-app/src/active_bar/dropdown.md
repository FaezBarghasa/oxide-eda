---
okf_version: "0.2"
type: Module
title: dropdown
description: "Active Bar dropdown menus — data-driven. Each `ActiveBarMenu` builds"
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown
language: rust
---

# dropdown

Active Bar dropdown menus — data-driven. Each `ActiveBarMenu` builds

## Docstring

Active Bar dropdown menus — data-driven. Each `ActiveBarMenu` builds
pure `DropdownEntry` rows that the shared
`oxide_widgets::active_bar_dropdown` widget renders, so the
schematic, footprint, and future PCB active bars share one dropdown
renderer (see ADR-0003). Enable/disable is folded into each
`DropdownItem` at build time, so no render-time selection guard is
needed here.

## Relationships

| Type | Target |
|------|--------|
| related | [view_dropdown](/crates/oxide-app/src/active_bar/dropdown/view_dropdown.md) |
| related | [dd_item](/crates/oxide-app/src/active_bar/dropdown/dd_item.md) |
| related | [dropdown_entries](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries.md) |
| related | [EntrySpec](/crates/oxide-app/src/active_bar/dropdown/EntrySpec.md) |
| related | [item](/crates/oxide-app/src/active_bar/dropdown/item.md) |
| related | [item](/crates/oxide-app/src/active_bar/dropdown/item.md) |
| related | [render](/crates/oxide-app/src/active_bar/dropdown/render.md) |
| related | [net_color_entries](/crates/oxide-app/src/active_bar/dropdown/net_color_entries.md) |
| related | [filter_entry](/crates/oxide-app/src/active_bar/dropdown/filter_entry.md) |
| related | [dropdown_min_width](/crates/oxide-app/src/active_bar/dropdown/dropdown_min_width.md) |
| related | [dropdown_x_offset](/crates/oxide-app/src/active_bar/dropdown/dropdown_x_offset.md) |
| related | [dd_btn_style_f](/crates/oxide-app/src/active_bar/dropdown/dd_btn_style_f.md) |
| related | [items](/crates/oxide-app/src/active_bar/dropdown/items.md) |
| related | [labels](/crates/oxide-app/src/active_bar/dropdown/labels.md) |
| related | [seps](/crates/oxide-app/src/active_bar/dropdown/seps.md) |
| related | [customs](/crates/oxide-app/src/active_bar/dropdown/customs.md) |
| related | [disabled_of](/crates/oxide-app/src/active_bar/dropdown/disabled_of.md) |
| related | [action_enable_predicate](/crates/oxide-app/src/active_bar/dropdown/action_enable_predicate.md) |
| related | [wiring_menu_is_four_ungated_items](/crates/oxide-app/src/active_bar/dropdown/wiring_menu_is_four_ungated_items.md) |
| related | [disabled_state_flips_but_labels_are_stable](/crates/oxide-app/src/active_bar/dropdown/disabled_state_flips_but_labels_are_stable.md) |
| related | [select_mode_toggle_sits_after_the_separator](/crates/oxide-app/src/active_bar/dropdown/select_mode_toggle_sits_after_the_separator.md) |
| related | [align_and_shapes_have_expected_shape](/crates/oxide-app/src/active_bar/dropdown/align_and_shapes_have_expected_shape.md) |
| related | [net_color_swatches_and_gated_clear_rows](/crates/oxide-app/src/active_bar/dropdown/net_color_swatches_and_gated_clear_rows.md) |
| related | [filter_menu_is_a_single_custom_entry](/crates/oxide-app/src/active_bar/dropdown/filter_menu_is_a_single_custom_entry.md) |
| related | [describe](/crates/oxide-app/src/active_bar/dropdown/describe.md) |
| related | [dropdown_entries_match_pre_refactor_golden](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries_match_pre_refactor_golden.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
