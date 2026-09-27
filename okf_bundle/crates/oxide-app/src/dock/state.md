---
okf_version: "0.2"
type: Module
title: state
description: "Dock area state: construction, panel management, and update handling."
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state
language: rust
---

# state

Dock area state: construction, panel management, and update handling.

## Docstring

Dock area state: construction, panel management, and update handling.

## Relationships

| Type | Target |
|------|--------|
| related | [default](/crates/oxide-app/src/dock/state/default.md) |
| related | [default](/crates/oxide-app/src/dock/state/default.md) |
| related | [new](/crates/oxide-app/src/dock/state/new.md) |
| related | [locate](/crates/oxide-app/src/dock/state/locate.md) |
| related | [add_panel](/crates/oxide-app/src/dock/state/add_panel.md) |
| related | [show_panel](/crates/oxide-app/src/dock/state/show_panel.md) |
| related | [show_panel_at](/crates/oxide-app/src/dock/state/show_panel_at.md) |
| related | [reveal_tab](/crates/oxide-app/src/dock/state/reveal_tab.md) |
| related | [region](/crates/oxide-app/src/dock/state/region.md) |
| related | [region_mut](/crates/oxide-app/src/dock/state/region_mut.md) |
| related | [update](/crates/oxide-app/src/dock/state/update.md) |
| related | [is_collapsed](/crates/oxide-app/src/dock/state/is_collapsed.md) |
| related | [has_panels](/crates/oxide-app/src/dock/state/has_panels.md) |
| related | [panel_kinds](/crates/oxide-app/src/dock/state/panel_kinds.md) |
| related | [active_tab](/crates/oxide-app/src/dock/state/active_tab.md) |
| related | [new](/crates/oxide-app/src/dock/state/new.md) |
| related | [locate](/crates/oxide-app/src/dock/state/locate.md) |
| related | [add_panel](/crates/oxide-app/src/dock/state/add_panel.md) |
| related | [show_panel](/crates/oxide-app/src/dock/state/show_panel.md) |
| related | [show_panel_at](/crates/oxide-app/src/dock/state/show_panel_at.md) |
| related | [reveal_tab](/crates/oxide-app/src/dock/state/reveal_tab.md) |
| related | [region](/crates/oxide-app/src/dock/state/region.md) |
| related | [region_mut](/crates/oxide-app/src/dock/state/region_mut.md) |
| related | [update](/crates/oxide-app/src/dock/state/update.md) |
| related | [is_collapsed](/crates/oxide-app/src/dock/state/is_collapsed.md) |
| related | [has_panels](/crates/oxide-app/src/dock/state/has_panels.md) |
| related | [panel_kinds](/crates/oxide-app/src/dock/state/panel_kinds.md) |
| related | [active_tab](/crates/oxide-app/src/dock/state/active_tab.md) |
| related | [floating](/crates/oxide-app/src/dock/state/floating.md) |
| related | [a_kind_docked_in_one_region_is_not_added_to_another](/crates/oxide-app/src/dock/state/a_kind_docked_in_one_region_is_not_added_to_another.md) |
| related | [replaying_a_duplicated_saved_layout_keeps_only_the_first_copy](/crates/oxide-app/src/dock/state/replaying_a_duplicated_saved_layout_keeps_only_the_first_copy.md) |
| related | [add_panel_leaves_an_existing_panel_where_it_is](/crates/oxide-app/src/dock/state/add_panel_leaves_an_existing_panel_where_it_is.md) |
| related | [show_panel_docks_a_new_kind_at_its_home_region_and_focuses_it](/crates/oxide-app/src/dock/state/show_panel_docks_a_new_kind_at_its_home_region_and_focuses_it.md) |
| related | [show_panel_reveals_a_panel_hidden_behind_another_tab](/crates/oxide-app/src/dock/state/show_panel_reveals_a_panel_hidden_behind_another_tab.md) |
| related | [show_panel_at_honours_an_explicit_drop_target_for_a_new_kind](/crates/oxide-app/src/dock/state/show_panel_at_honours_an_explicit_drop_target_for_a_new_kind.md) |
| related | [a_floating_panel_blocks_a_second_docked_copy](/crates/oxide-app/src/dock/state/a_floating_panel_blocks_a_second_docked_copy.md) |
| related | [locate_finds_a_docked_panel_by_region_and_index](/crates/oxide-app/src/dock/state/locate_finds_a_docked_panel_by_region_and_index.md) |
| related | [undocking_then_reopening_does_not_produce_a_second_copy](/crates/oxide-app/src/dock/state/undocking_then_reopening_does_not_produce_a_second_copy.md) |
| related | [re_docking_a_floating_panel_sends_it_to_its_home_region](/crates/oxide-app/src/dock/state/re_docking_a_floating_panel_sends_it_to_its_home_region.md) |
| related | [dropping_a_floating_panel_on_a_zone_docks_it_there](/crates/oxide-app/src/dock/state/dropping_a_floating_panel_on_a_zone_docks_it_there.md) |
