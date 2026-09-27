---
okf_version: "0.2"
type: Function
title: build_export_scope
description: "The export's page set + metadata + project netlist, alongside the stitch"
resource: crates/oxide-app/src/app/handlers/menu/export/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope
language: rust
---

# build_export_scope

The export's page set + metadata + project netlist, alongside the stitch

## Signature

```rust
fn build_export_scope(
    document_state: &crate::app::state::DocumentState,
) -> Option<(ExportContext, ExportIssues)>
```

## Docstring

The export's page set + metadata + project netlist, alongside the stitch
issues raised while deriving that netlist. Returns `None` if there is no
active engine.

The issues are handed back rather than acted on here, because severity is
a **per-deliverable policy**, not a property of the scope: the `.net` file
is machine-consumed and refuses on a hole, the PDF is human-consumed and
degrades loudly. See `handle_export_netlist_finished` /
`handle_export_pdf_finished`.

## Source
Lines 241–350 in `crates/oxide-app/src/app/handlers/menu/export/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [export](/crates/oxide-app/src/app/handlers/menu/export/mod.md) |
| calls | [assemble_active_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets.md) |
| calls | [owned_pages](/crates/oxide-app/src/app/handlers/menu/export/mod/owned_pages.md) |
| calls | [loose_pages](/crates/oxide-app/src/app/handlers/menu/export/mod/loose_pages.md) |
| calls | [export_metadata](/crates/oxide-app/src/app/handlers/menu/export/mod/export_metadata.md) |
| calls | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |
| calls | [sheet_key](/crates/oxide-app/src/app/project_sheets/sheet_key.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| calls | [project_roots](/crates/oxide-app/src/app/project_sheets/project_roots.md) |
| calls | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
| called_by | [handle_export_bom_finished](/crates/oxide-app/src/app/handlers/menu/export/bom/handle_export_bom_finished.md) |
| called_by | [build_export_context](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_context.md) |
| called_by | [handle_export_netlist_finished](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_export_netlist_finished.md) |
| called_by | [handle_export_pdf_finished](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_export_pdf_finished.md) |
| called_by | [handle_print_preview_requested](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_requested.md) |
| called_by | [a_child_only_on_disk_is_stitched_without_being_opened](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_only_on_disk_is_stitched_without_being_opened.md) |
| called_by | [a_child_that_exists_but_will_not_parse_is_not_called_missing](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_that_exists_but_will_not_parse_is_not_called_missing.md) |
| called_by | [a_flat_projects_second_page_no_longer_vanishes_from_the_netlist](/crates/oxide-app/src/app/handlers/menu/export/tests/a_flat_projects_second_page_no_longer_vanishes_from_the_netlist.md) |
| called_by | [a_page_the_root_does_reach_is_not_reported_as_a_shortfall](/crates/oxide-app/src/app/handlers/menu/export/tests/a_page_the_root_does_reach_is_not_reported_as_a_shortfall.md) |
| called_by | [context](/crates/oxide-app/src/app/handlers/menu/export/tests/context.md) |
| called_by | [a_listed_page_with_no_file_is_not_diagnosed_as_a_graph_problem](/crates/oxide-app/src/app/handlers/menu/export/tests/diagnosis/a_listed_page_with_no_file_is_not_diagnosed_as_a_graph_problem.md) |
| called_by | [hierarchical_child_sheet_exports_its_owning_project](/crates/oxide-app/src/app/handlers/menu/export/tests/hierarchical_child_sheet_exports_its_owning_project.md) |
| called_by | [root_active_netlist_contains_a_child_absent_from_data_sheets](/crates/oxide-app/src/app/handlers/menu/export/tests/root_active_netlist_contains_a_child_absent_from_data_sheets.md) |
