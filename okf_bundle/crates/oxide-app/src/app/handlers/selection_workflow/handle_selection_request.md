---
okf_version: "0.2"
type: Function
title: handle_selection_request
resource: crates/oxide-app/src/app/handlers/selection_workflow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/selection_workflow/handle_selection_request
language: rust
---

# handle_selection_request

## Signature

```rust
impl Oxide { pub(crate) fn handle_selection_request(
        &mut self,
        request: selection_request::SelectionRequest,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 120–213 in `crates/oxide-app/src/app/handlers/selection_workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_workflow](/crates/oxide-app/src/app/handlers/selection_workflow.md) |
| calls | [all_selectable_items](/crates/oxide-app/src/app/handlers/selection_workflow/all_selectable_items.md) |
| calls | [valid_selection_items](/crates/oxide-app/src/app/handlers/selection_workflow/valid_selection_items.md) |
| calls | [passes_filter](/crates/oxide-app/src/app/handlers/selection_workflow/passes_filter.md) |
| calls | [hit_test_rect_mode](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_rect_mode.md) |
| calls | [expand_to_net](/crates/oxide-app/src/app/handlers/selection_workflow/expand_to_net.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
