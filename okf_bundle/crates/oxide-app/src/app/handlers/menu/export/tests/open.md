---
okf_version: "0.2"
type: Function
title: open
description: "Open `path` as a live engine whose sheet references `children`."
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/open
language: rust
---

# open

Open `path` as a live engine whose sheet references `children`.

## Signature

```rust
fn open(ds: &mut DocumentState, path: &Path, children: &[&str])
```

## Docstring

Open `path` as a live engine whose sheet references `children`.

## Source
Lines 238–240 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| calls | [open_with](/crates/oxide-app/src/app/handlers/menu/export/tests/open_with.md) |
| calls | [schematic](/crates/oxide-app/src/app/handlers/menu/export/tests/schematic.md) |
| called_by | [a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths](/crates/oxide-app/src/app/handlers/menu/export/tests/a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths.md) |
| called_by | [grandchild_resolves_through_two_hops_to_the_project](/crates/oxide-app/src/app/handlers/menu/export/tests/grandchild_resolves_through_two_hops_to_the_project.md) |
| called_by | [hierarchical_child_sheet_exports_its_owning_project](/crates/oxide-app/src/app/handlers/menu/export/tests/hierarchical_child_sheet_exports_its_owning_project.md) |
| called_by | [listed_project_sheet_exports_the_whole_project](/crates/oxide-app/src/app/handlers/menu/export/tests/listed_project_sheet_exports_the_whole_project.md) |
| called_by | [loose_export_page_order_is_stable_across_rebuilds](/crates/oxide-app/src/app/handlers/menu/export/tests/loose_export_page_order_is_stable_across_rebuilds.md) |
| called_by | [loose_schematic_exports_itself_not_the_sticky_projects_sheets](/crates/oxide-app/src/app/handlers/menu/export/tests/loose_schematic_exports_itself_not_the_sticky_projects_sheets.md) |
| called_by | [schematic_inside_the_project_directory_but_unlisted_stays_loose](/crates/oxide-app/src/app/handlers/menu/export/tests/schematic_inside_the_project_directory_but_unlisted_stays_loose.md) |
