---
okf_version: "0.2"
type: Function
title: project_owning_sheet
description: "Project that owns `path`, directly or as a hierarchical descendant of one"
resource: crates/oxide-app/src/app/state/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:25Z"
concept_id: crates/oxide-app/src/app/state/scope/project_owning_sheet
language: rust
---

# project_owning_sheet

Project that owns `path`, directly or as a hierarchical descendant of one

## Signature

```rust
pub(crate) fn project_owning_sheet(
    projects: &'a [LoadedProject],
    loaded: &HashMap<PathBuf, Vec<String>>,
    path: &Path,
) -> Option<&'a LoadedProject>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Docstring

Project that owns `path`, directly or as a hierarchical descendant of one
of its sheets. `loaded` maps each open sheet's path to the `filename`
strings it references as children.

Cycles in the hierarchy (a child re-referencing an ancestor) terminate on
the visited set rather than looping — ERC reports those separately.

## Source
Lines 92–111 in `crates/oxide-app/src/app/state/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-app/src/app/state/scope.md) |
| calls | [project_listing_sheet](/crates/oxide-app/src/app/state/scope/project_listing_sheet.md) |
| calls | [parent_of](/crates/oxide-app/src/app/state/scope/parent_of.md) |
| calls | [path_key](/crates/oxide-app/src/app/state/scope/path_key.md) |
| called_by | [active_document_project](/crates/oxide-app/src/app/state/mod/active_document_project.md) |
| called_by | [unowned_engine_paths](/crates/oxide-app/src/app/state/mod/unowned_engine_paths.md) |
| called_by | [grandchild_resolves_through_two_hops](/crates/oxide-app/src/app/state/scope/grandchild_resolves_through_two_hops.md) |
| called_by | [hierarchical_child_resolves_to_the_project_of_its_root](/crates/oxide-app/src/app/state/scope/hierarchical_child_resolves_to_the_project_of_its_root.md) |
| called_by | [owning_project_is_the_one_listing_the_sheet](/crates/oxide-app/src/app/state/scope/owning_project_is_the_one_listing_the_sheet.md) |
| called_by | [path_case_and_separator_do_not_change_ownership_on_windows](/crates/oxide-app/src/app/state/scope/path_case_and_separator_do_not_change_ownership_on_windows.md) |
| called_by | [shared_child_ownership_is_deterministic_regardless_of_map_order](/crates/oxide-app/src/app/state/scope/shared_child_ownership_is_deterministic_regardless_of_map_order.md) |
