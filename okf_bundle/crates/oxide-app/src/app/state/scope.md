---
okf_version: "0.2"
type: Module
title: scope
description: Which project the document on screen belongs to.
resource: crates/oxide-app/src/app/state/scope.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:25Z"
concept_id: crates/oxide-app/src/app/state/scope
language: rust
---

# scope

Which project the document on screen belongs to.

## Docstring

Which project the document on screen belongs to.

`DocumentState.active_project` is a *sticky* workspace pointer: it keeps
naming the last-loaded project while the user focuses a tab that has no
project of its own (a loose schematic, a symbol / footprint editor). Every
subsystem that operates on "the sheet set the user is working in" — export,
ERC, annotate, hierarchical child-sheet resolution — must instead resolve
from the *active document*, or it silently pulls another project's sheets
into the run (#406).

Membership is decided two ways, in order:

1. the path is in the project's persisted `sheets` list (`.snxprj`);
2. the path is reachable from a listed sheet by following the loaded
`child_sheets` graph. Descending into a hierarchical child opens it as a
tab without adding it to `sheets`, so rule 1 alone reports every child
sheet as loose.

## Relationships

| Type | Target |
|------|--------|
| related | [path_key](/crates/oxide-app/src/app/state/scope/path_key.md) |
| related | [project_listing_sheet](/crates/oxide-app/src/app/state/scope/project_listing_sheet.md) |
| related | [parent_of](/crates/oxide-app/src/app/state/scope/parent_of.md) |
| related | [project_owning_sheet](/crates/oxide-app/src/app/state/scope/project_owning_sheet.md) |
| related | [project](/crates/oxide-app/src/app/state/scope/project.md) |
| related | [loaded](/crates/oxide-app/src/app/state/scope/loaded.md) |
| related | [owning_project_is_the_one_listing_the_sheet](/crates/oxide-app/src/app/state/scope/owning_project_is_the_one_listing_the_sheet.md) |
| related | [loose_sheet_in_another_directory_belongs_to_no_project](/crates/oxide-app/src/app/state/scope/loose_sheet_in_another_directory_belongs_to_no_project.md) |
| related | [sheet_inside_the_project_directory_but_unlisted_belongs_to_no_project](/crates/oxide-app/src/app/state/scope/sheet_inside_the_project_directory_but_unlisted_belongs_to_no_project.md) |
| related | [hierarchical_child_resolves_to_the_project_of_its_root](/crates/oxide-app/src/app/state/scope/hierarchical_child_resolves_to_the_project_of_its_root.md) |
| related | [grandchild_resolves_through_two_hops](/crates/oxide-app/src/app/state/scope/grandchild_resolves_through_two_hops.md) |
| related | [a_hierarchy_cycle_terminates_instead_of_looping](/crates/oxide-app/src/app/state/scope/a_hierarchy_cycle_terminates_instead_of_looping.md) |
| related | [shared_child_ownership_is_deterministic_regardless_of_map_order](/crates/oxide-app/src/app/state/scope/shared_child_ownership_is_deterministic_regardless_of_map_order.md) |
| related | [path_case_and_separator_do_not_change_ownership_on_windows](/crates/oxide-app/src/app/state/scope/path_case_and_separator_do_not_change_ownership_on_windows.md) |
