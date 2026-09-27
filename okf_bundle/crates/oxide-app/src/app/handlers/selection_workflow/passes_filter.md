---
okf_version: "0.2"
type: Function
title: passes_filter
description: "Return `true` iff the currently active filter set allows selecting the"
resource: crates/oxide-app/src/app/handlers/selection_workflow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/selection_workflow/passes_filter
language: rust
---

# passes_filter

Return `true` iff the currently active filter set allows selecting the

## Signature

```rust
pub(crate) fn passes_filter(
    item: &oxide_types::schematic::SelectedItem,
    snapshot: &crate::schematic_runtime::SchematicRenderSnapshot,
    filters: &std::collections::HashSet<SelectionFilter>,
) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

Return `true` iff the currently active filter set allows selecting the
given hit. When no filters are active (empty set), selection is blocked
entirely — that matches the Altium "unselect all categories" behaviour.

## Source
Lines 9–39 in `crates/oxide-app/src/app/handlers/selection_workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_workflow](/crates/oxide-app/src/app/handlers/selection_workflow.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [dispatch_update](/crates/oxide-app/src/app/dispatch/mod/dispatch_update.md) |
| called_by | [handle_canvas_clicked](/crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked.md) |
| called_by | [handle_canvas_interaction_event](/crates/oxide-app/src/app/handlers/canvas/mod/handle_canvas_interaction_event.md) |
| called_by | [handle_selection_request](/crates/oxide-app/src/app/handlers/selection_workflow/handle_selection_request.md) |
