---
okf_version: "0.2"
type: Function
title: build_export_context
description: "Assemble the export context, discarding the stitch issues — for the"
resource: crates/oxide-app/src/app/handlers/menu/export/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/mod/build_export_context
language: rust
---

# build_export_context

Assemble the export context, discarding the stitch issues — for the

## Signature

```rust
fn build_export_context(
    document_state: &crate::app::state::DocumentState,
) -> Option<ExportContext>
```

## Docstring

Assemble the export context, discarding the stitch issues — for the
callers that render an *intermediate* artifact rather than a deliverable
(the BOM dialog, every print-preview rerasterize).

Deliberately silent. Round 3 logged each issue here, and
`rerasterize_print_preview` calls this on every settings toggle *and* on
every keystroke in the specific-page input; the Messages panel keeps only
`MAX_DIAGNOSTIC_ENTRIES` (200) with no dedupe, so those copies evicted the
user's ERC results and eventually the earliest copies of the very stitch
warnings this path exists to deliver. Surfacing belongs to the user
actions — see [`log_stitch_issues`].

## Source
Lines 24–28 in `crates/oxide-app/src/app/handlers/menu/export/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [export](/crates/oxide-app/src/app/handlers/menu/export/mod.md) |
| calls | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| called_by | [rebuild_bom_table](/crates/oxide-app/src/app/handlers/menu/export/bom/rebuild_bom_table.md) |
| called_by | [rerasterize_print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview/rerasterize_print_preview.md) |
| called_by | [rerasterizing_the_preview_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/handlers/menu/export/tests/rerasterizing_the_preview_does_not_flood_the_messages_panel.md) |
