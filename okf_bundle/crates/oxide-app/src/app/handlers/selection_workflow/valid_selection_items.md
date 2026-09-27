---
okf_version: "0.2"
type: Function
title: valid_selection_items
resource: crates/oxide-app/src/app/handlers/selection_workflow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/selection_workflow/valid_selection_items
language: rust
---

# valid_selection_items

## Signature

```rust
fn valid_selection_items(
    snapshot: &crate::schematic_runtime::SchematicRenderSnapshot,
    items: &[oxide_types::schematic::SelectedItem],
) -> Vec<oxide_types::schematic::SelectedItem>
```

## Source
Lines 94–117 in `crates/oxide-app/src/app/handlers/selection_workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_workflow](/crates/oxide-app/src/app/handlers/selection_workflow.md) |
| calls | [all_selectable_items](/crates/oxide-app/src/app/handlers/selection_workflow/all_selectable_items.md) |
| called_by | [handle_selection_request](/crates/oxide-app/src/app/handlers/selection_workflow/handle_selection_request.md) |
