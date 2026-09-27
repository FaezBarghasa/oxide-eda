---
okf_version: "0.2"
type: Function
title: build_footprint_editor_panel_ctx
description: "v0.14.2 — project the active `.snxfpt` editor's data into a"
resource: crates/oxide-app/src/app/runtime/footprint_ctx.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx
language: rust
---

# build_footprint_editor_panel_ctx

v0.14.2 — project the active `.snxfpt` editor's data into a

## Signature

```rust
pub(super) fn build_footprint_editor_panel_ctx(
    app: &super::super::Oxide,
) -> Option<crate::panels::FootprintEditorPanelContext>
```

## Visibility

- `pub(super)`

## Docstring

v0.14.2 — project the active `.snxfpt` editor's data into a
panel-side snapshot. Mirrors `build_symbol_editor_panel_ctx`.

## Source
Lines 8–639 in `crates/oxide-app/src/app/runtime/footprint_ctx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx.md) |
| calls | [build_over_constraint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries/build_over_constraint_summaries.md) |
| calls | [footprint_pad_kind_label](/crates/oxide-app/src/app/runtime/footprint_summaries/footprint_pad_kind_label.md) |
| calls | [footprint_pad_shape_label](/crates/oxide-app/src/app/runtime/footprint_summaries/footprint_pad_shape_label.md) |
| calls | [build_sketch_entity_summary](/crates/oxide-app/src/app/runtime/footprint_summaries/build_sketch_entity_summary.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [list_dir_or_report](/crates/oxide-app/src/app/dir_listing/list_dir_or_report.md) |
| calls | [current_role_of](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/current_role_of.md) |
| called_by | [refresh_panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx.md) |
