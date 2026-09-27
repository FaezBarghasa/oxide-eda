---
okf_version: "0.2"
type: Function
title: all_selectable_items
resource: crates/oxide-app/src/app/handlers/selection_workflow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/selection_workflow/all_selectable_items
language: rust
---

# all_selectable_items

## Signature

```rust
fn all_selectable_items(
    snapshot: &crate::schematic_runtime::SchematicRenderSnapshot,
) -> Vec<oxide_types::schematic::SelectedItem>
```

## Source
Lines 41–92 in `crates/oxide-app/src/app/handlers/selection_workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_workflow](/crates/oxide-app/src/app/handlers/selection_workflow.md) |
| called_by | [handle_selection_request](/crates/oxide-app/src/app/handlers/selection_workflow/handle_selection_request.md) |
| called_by | [valid_selection_items](/crates/oxide-app/src/app/handlers/selection_workflow/valid_selection_items.md) |
