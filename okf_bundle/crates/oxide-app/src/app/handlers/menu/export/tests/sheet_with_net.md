---
okf_version: "0.2"
type: Function
title: sheet_with_net
description: "A sheet with one component pin sitting on a wire named by a `Global` label."
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/sheet_with_net
language: rust
---

# sheet_with_net

A sheet with one component pin sitting on a wire named by a `Global` label.

## Signature

```rust
pub(crate) fn sheet_with_net(reference: &str, net_name: &str, children: &[&str]) -> SchematicSheet
```

## Visibility

- `pub(crate)`

## Docstring

A sheet with one component pin sitting on a wire named by a `Global` label.

Both halves matter: a net only exists where a terminal lands, so the pin is
what makes the net real, and the reference is what a dropped subtree costs
you on the board — missing components. `net_name` is unqualified in the
project netlist because the label is `Global`.

## Source
Lines 81–173 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| calls | [schematic](/crates/oxide-app/src/app/handlers/menu/export/tests/schematic.md) |
| called_by | [a_child_only_on_disk_is_stitched_without_being_opened](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_only_on_disk_is_stitched_without_being_opened.md) |
| called_by | [a_child_that_exists_but_will_not_parse_is_not_called_missing](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_that_exists_but_will_not_parse_is_not_called_missing.md) |
| called_by | [a_page_the_root_does_reach_is_not_reported_as_a_shortfall](/crates/oxide-app/src/app/handlers/menu/export/tests/a_page_the_root_does_reach_is_not_reported_as_a_shortfall.md) |
| called_by | [a_project_export_roots_at_the_project_root_not_the_active_child](/crates/oxide-app/src/app/handlers/menu/export/tests/a_project_export_roots_at_the_project_root_not_the_active_child.md) |
| called_by | [app_flat_project](/crates/oxide-app/src/app/handlers/menu/export/tests/app_flat_project.md) |
| called_by | [app_with_missing_child](/crates/oxide-app/src/app/handlers/menu/export/tests/app_with_missing_child.md) |
| called_by | [a_listed_page_with_no_file_is_not_diagnosed_as_a_graph_problem](/crates/oxide-app/src/app/handlers/menu/export/tests/diagnosis/a_listed_page_with_no_file_is_not_diagnosed_as_a_graph_problem.md) |
| called_by | [root_active_netlist_contains_a_child_absent_from_data_sheets](/crates/oxide-app/src/app/handlers/menu/export/tests/root_active_netlist_contains_a_child_absent_from_data_sheets.md) |
| called_by | [app_with_a_child_only_on_disk](/crates/oxide-app/src/app/mutation_gateway/app_with_a_child_only_on_disk.md) |
| called_by | [resetting_duplicate_designators_sees_a_child_that_is_only_on_disk](/crates/oxide-app/src/app/mutation_gateway/resetting_duplicate_designators_sees_a_child_that_is_only_on_disk.md) |
| called_by | [fixture](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/fixture.md) |
