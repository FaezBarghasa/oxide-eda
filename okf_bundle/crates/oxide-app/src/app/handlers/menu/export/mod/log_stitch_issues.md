---
okf_version: "0.2"
type: Function
title: log_stitch_issues
description: "Put the stitch issues in front of the user — called once per *user action*"
resource: crates/oxide-app/src/app/handlers/menu/export/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/mod/log_stitch_issues
language: rust
---

# log_stitch_issues

Put the stitch issues in front of the user — called once per *user action*

## Signature

```rust
fn log_stitch_issues(
    document_state: &crate::app::state::DocumentState,
    ctx: &ExportContext,
    issues: &ExportIssues,
)
```

## Docstring

Put the stitch issues in front of the user — called once per *user action*
(print-preview open, PDF written, netlist written), never from the shared
context builder.

## Source
Lines 108–140 in `crates/oxide-app/src/app/handlers/menu/export/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [export](/crates/oxide-app/src/app/handlers/menu/export/mod.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| called_by | [handle_export_bom_finished](/crates/oxide-app/src/app/handlers/menu/export/bom/handle_export_bom_finished.md) |
| called_by | [handle_export_netlist_finished](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_export_netlist_finished.md) |
| called_by | [handle_export_pdf_finished](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_export_pdf_finished.md) |
| called_by | [handle_print_preview_requested](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_requested.md) |
