---
okf_version: "0.2"
type: Function
title: handle_add_new_schematic_picked
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/add/handle_add_new_schematic_picked_1
language: rust
---

# handle_add_new_schematic_picked

## Signature

```rust
pub(crate) fn handle_add_new_schematic_picked(
        &mut self,
        project_idx: usize,
        path: Option<std::path::PathBuf>,
    )
```

## Visibility

- `pub(crate)`

## Source
Lines 160–237 in `crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [add](/crates/oxide-app/src/app/handlers/dock/project_navigation/add.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
| calls | [blank_schematic_sheet_for_new_doc](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/blank_schematic_sheet_for_new_doc.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
