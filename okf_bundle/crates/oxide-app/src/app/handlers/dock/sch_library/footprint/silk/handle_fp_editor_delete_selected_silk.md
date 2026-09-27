---
okf_version: "0.2"
type: Function
title: handle_fp_editor_delete_selected_silk
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_delete_selected_silk
language: rust
---

# handle_fp_editor_delete_selected_silk

## Signature

```rust
impl Oxide { pub(in crate::app::handlers::dock::sch_library) fn handle_fp_editor_delete_selected_silk(
        &mut self,
    ) -> bool }
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Source
Lines 150–177 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [silk](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [adjust_selection_after_remove](/crates/oxide-app/src/library/editor/footprint/state/mod/adjust_selection_after_remove.md) |
